//! edge-kg-agent — bootstrap and wire everything together.

mod agents;
mod config;
mod db;
mod error;
mod ingest;
mod limiter;
mod orchestrator;
mod scheduler;
mod status;
mod telemetry;
mod vault;
mod watcher;
mod workflow;

use std::process::ExitCode;

use tokio::signal;
#[cfg(unix)]
use tracing::error;
use tracing::info;

use crate::{
    agents::{
        bridge::BridgeFinderAgent, curator::TopicCuratorAgent,
        harvester::FormulaHarvesterAgent, search::LiteratureSearchAgent,
        KnowledgeGraphAgent,
    },
    config::Config,
    db::Db,
    error::AppResult,
    limiter::Limiter,
    orchestrator::Orchestrator,
    scheduler::Scheduler,
    vault::VaultWriter,
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
        reasoner = %cfg.reasoner_model,
        vision = %cfg.vision_model,
        rpm = cfg.rpm_limit,
        index_cap = cfg.index_entry_cap,
        "starting edge-kg-agent"
    );

    let db = Db::open(&cfg.db_path).await?;
    let requeued = db.requeue_orphans().await?;
    if requeued > 0 {
        info!(requeued, "recovered orphaned batches from previous run");
    }

    // Core agents.
    let kg = KnowledgeGraphAgent::new(&cfg)?;
    let vault = VaultWriter::new(cfg.vault_dir.clone(), cfg.index_entry_cap);

    // Shared role-aware limiter — the only gate every non-extractor LLM
    // call must pass through. (Extractor keeps its own governor for now;
    // that flip lands in a follow-up change.)
    let limiter = Limiter::from_config(&cfg)?;

    // Research-stack agents. Each wraps its own `Arc<Google>` client
    // keyed on its role's model override, but all share `limiter`.
    let search_agent = LiteratureSearchAgent::new(&cfg, db.clone())?;
    let curator_agent = TopicCuratorAgent::new(&cfg, limiter.clone())?;
    let bridge_agent =
        BridgeFinderAgent::new(&cfg, limiter.clone(), search_agent)?;
    let harvester_agent =
        FormulaHarvesterAgent::new(cfg.vault_dir.clone(), db.clone())?;
    let scheduler = Scheduler::new(cfg.clone(), db.clone())?;

    let orch = Orchestrator::new(cfg.clone(), db.clone(), kg, vault);

    // Status heartbeat.
    status::spawn(db.clone(), cfg.vault_dir.clone(), cfg.status_interval);

    // Filesystem watcher → orchestrator ingest consumer.
    let rx = watcher::spawn(cfg.watch_dir.clone(), cfg.fs_debounce)?;
    orch.spawn_ingest(rx);
    orch.spawn_reasoner();
    orch.spawn_research(curator_agent, bridge_agent);
    orch.spawn_harvester(harvester_agent);
    orch.spawn_idle_scheduler(scheduler);

    shutdown_signal().await;
    info!("shutdown signal received");
    Ok(())
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
