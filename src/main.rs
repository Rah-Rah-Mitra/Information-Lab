//! edge-kg-agent — bootstrap and wire everything together.

mod agents;
mod config;
mod db;
mod error;
mod ingest;
mod orchestrator;
mod status;
mod tools;
mod vault;
mod watcher;

use std::process::ExitCode;

use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::{
    agents::{KnowledgeGraphAgent, ResearchStack},
    config::Config,
    db::Db,
    error::AppResult,
    orchestrator::Orchestrator,
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
    init_tracing(&cfg)?;

    info!(
        watch = %cfg.watch_dir.display(),
        vault = %cfg.vault_dir.display(),
        db = %cfg.db_path.display(),
        reasoner = %cfg.reasoner_model,
        vision = %cfg.vision_model,
        rpm = cfg.rpm_limit,
        research = cfg.tavily_key.is_some(),
        "starting edge-kg-agent"
    );

    let db = Db::open(&cfg.db_path).await?;
    let requeued = db.requeue_orphans().await?;
    if requeued > 0 {
        info!(requeued, "recovered orphaned batches from previous run");
    }

    // Core agents.
    let kg = KnowledgeGraphAgent::new(&cfg)?;
    let research = match ResearchStack::new(&cfg, cfg.tavily_key.clone()) {
        Ok(r) => Some(r),
        Err(e) => {
            error!(error = %e, "research stack disabled");
            None
        }
    };
    let vault = VaultWriter::new(cfg.vault_dir.clone());

    let orch = Orchestrator::new(cfg.clone(), db.clone(), kg, research, vault);

    // Status heartbeat.
    status::spawn(db.clone(), cfg.vault_dir.clone(), cfg.status_interval);

    // Filesystem watcher → orchestrator ingest consumer.
    let rx = watcher::spawn(cfg.watch_dir.clone(), cfg.fs_debounce)?;
    orch.spawn_ingest(rx);
    orch.spawn_reasoner();

    shutdown_signal().await;
    info!("shutdown signal received");
    Ok(())
}

fn init_tracing(cfg: &Config) -> AppResult<()> {
    std::fs::create_dir_all(&cfg.log_dir)?;
    let file_appender = tracing_appender::rolling::daily(&cfg.log_dir, "edge-kg-agent.log");
    let (nb_file, guard) = tracing_appender::non_blocking(file_appender);
    // The guard must outlive the process so the background writer keeps flushing.
    Box::leak(Box::new(guard));

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("edge_kg_agent=info,warn"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).with_ansi(true))
        .with(fmt::layer().json().with_writer(nb_file))
        .init();
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
