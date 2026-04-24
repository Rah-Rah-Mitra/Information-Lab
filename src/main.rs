//! edge-kg-agent — bootstrap and wire everything together.

use std::{collections::BTreeSet, process::ExitCode, time::Duration};

use tokio::signal;
#[cfg(unix)]
use tracing::error;
use tracing::{info, warn};

use edge_kg_agent::{
    agents::{
        bridge::BridgeFinderAgent, curator::TopicCuratorAgent, derivation::DerivationChainAgent,
        formula_extractor::FormulaExtractorAgent, harvester::FormulaHarvesterAgent,
        report::ReportWriterAgent, retrier::ErrorRetrierAgent, search::LiteratureSearchAgent,
        theorem::TheoremProverAgent, KnowledgeGraphAgent,
    },
    api,
    config::Config,
    db::Db,
    error::AppResult,
    limiter::Limiter,
    orchestrator::Orchestrator,
    scheduler::Scheduler,
    status, telemetry,
    vault::VaultWriter,
    watcher,
};

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            // Surface both Display and Debug for the systemd journal.
            eprintln!("fatal: {e}");
            eprintln!("debug: {e:?}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> AppResult<()> {
    let cfg = Config::from_env()?;
    let telemetry_guard = telemetry::init(&cfg)?;
    // Keep the guard alive for the lifetime of the process so both the
    // non-blocking file writer and the OTel batch exporter can flush.
    Box::leak(Box::new(telemetry_guard));

    info!(
        watch = %cfg.watch_dir.display(),
        vault = %cfg.vault_dir.display(),
        db = %cfg.db_path.display(),
        light = %cfg.light_model,
        heavy = %cfg.heavy_model,
        vision = %cfg.vision_model,
        rpm = cfg.rpm_limit,
        index_cap = cfg.index_entry_cap,
        "starting edge-kg-agent"
    );

    log_effective_models(&cfg);
    validate_models(&cfg).await?;

    let db = Db::open(
        &cfg.db_path,
        &cfg.thinking_redaction_policy,
        cfg.thinking_max_bytes,
    )
    .await?;
    let requeued = db.requeue_orphans().await?;
    if requeued > 0 {
        info!(requeued, "recovered orphaned batches from previous run");
    }

    // Shared dual-tier role-aware limiter. Every LLM call — extractor on
    // the light tier, curator/bridge on the heavy tier — gates through this.
    let limiter = Limiter::from_config(&cfg)?;

    // Core agents.
    let kg = KnowledgeGraphAgent::new(&cfg, limiter.clone(), db.clone())?;
    let vault = VaultWriter::new(cfg.vault_dir.clone(), cfg.index_entry_cap);

    // Research-stack agents. Each wraps its own `Arc<Google>` client
    // keyed on its role's model override, but all share `limiter`.
    let search_agent = LiteratureSearchAgent::new(&cfg, db.clone())?;
    let curator_agent = TopicCuratorAgent::new(&cfg, limiter.clone(), db.clone())?;
    let bridge_agent = BridgeFinderAgent::new(&cfg, limiter.clone(), db.clone(), search_agent)?;
    let harvester_agent = FormulaHarvesterAgent::new(cfg.vault_dir.clone(), db.clone())?;
    let retrier_agent = ErrorRetrierAgent::new(&cfg, db.clone());
    let theorem_agent = TheoremProverAgent::new(&cfg, limiter.clone(), db.clone())?;
    let derivation_agent = DerivationChainAgent::new(&cfg, limiter.clone(), db.clone())?;
    let report_agent = ReportWriterAgent::new(&cfg, limiter.clone(), db.clone())?;
    let formula_agent = FormulaExtractorAgent::new(&cfg, limiter.clone(), db.clone())?;
    let scheduler = Scheduler::new(cfg.clone(), db.clone())?;

    let orch = Orchestrator::new(cfg.clone(), db.clone(), kg, vault);

    // Status heartbeat.
    status::spawn(db.clone(), cfg.vault_dir.clone(), cfg.status_interval);
    api::spawn(db.clone(), cfg.research_api_bind.clone());

    // Filesystem watcher → orchestrator ingest consumer.
    let rx = watcher::spawn(cfg.watch_dir.clone(), cfg.fs_debounce)?;
    orch.spawn_ingest(rx);
    orch.spawn_reasoner();
    orch.spawn_research(curator_agent, bridge_agent);
    orch.spawn_harvester(harvester_agent);
    orch.spawn_error_retrier(retrier_agent);
    orch.spawn_heavy_research(theorem_agent, derivation_agent, report_agent);
    orch.spawn_formula_extractor(formula_agent);
    orch.spawn_idle_scheduler(scheduler);

    shutdown_signal().await;
    info!("shutdown signal received");
    Ok(())
}

fn log_effective_models(cfg: &Config) {
    info!(
        light = %cfg.light_model,
        heavy = %cfg.heavy_model,
        reasoner_alias = %cfg.reasoner_model,
        vision = %cfg.vision_model,
        formula = %cfg.model_for_override(&cfg.formula_model, &cfg.light_model),
        curator = %cfg.model_for_role(&cfg.curator_model),
        bridge = %cfg.model_for_role(&cfg.bridge_model),
        theorem = %cfg.model_for_role(&cfg.theorem_model),
        derivation = %cfg.model_for_role(&cfg.derivation_model),
        report = %cfg.model_for_role(&cfg.report_model),
        "effective model configuration"
    );
}

async fn validate_models(cfg: &Config) -> AppResult<()> {
    let mut models = BTreeSet::new();
    models.insert(cfg.light_model.clone());
    models.insert(cfg.heavy_model.clone());
    models.insert(cfg.vision_model.clone());
    models.insert(cfg.model_for_override(&cfg.formula_model, &cfg.light_model));
    models.insert(cfg.model_for_role(&cfg.curator_model));
    models.insert(cfg.model_for_role(&cfg.bridge_model));
    models.insert(cfg.model_for_role(&cfg.theorem_model));
    models.insert(cfg.model_for_role(&cfg.derivation_model));
    models.insert(cfg.model_for_role(&cfg.report_model));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .build()?;
    let base = cfg.api_base.trim_end_matches('/');

    let mut missing = Vec::new();
    for model in models {
        let url = format!("{base}/models/{model}");
        let resp = client
            .get(&url)
            .query(&[("key", cfg.api_key.as_str())])
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            missing.push(model);
            continue;
        }
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| String::new());
            warn!(
                status = %status,
                body = %truncate_for_log(&body, 200),
                "model preflight returned non-success"
            );
        }
    }

    if !missing.is_empty() {
        return Err(edge_kg_agent::error::AppError::other(format!(
            "model preflight failed: these model names returned 404: {}",
            missing.join(", ")
        )));
    }

    info!("model preflight ok");
    Ok(())
}

fn truncate_for_log(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &s[..end])
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut term = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                error!(error = %e, "failed to install SIGTERM handler");
                return;
            }
        };
        tokio::select! {
            _ = signal::ctrl_c() => {}
            _ = term.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        let _ = signal::ctrl_c().await;
    }
}
