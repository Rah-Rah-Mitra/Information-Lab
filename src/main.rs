//! edge-kg-agent — bootstrap and wire everything together.

mod agents;
mod config;
mod db;
mod error;
mod ingest;
mod orchestrator;
mod status;
mod telemetry;
mod vault;
mod watcher;

use std::process::ExitCode;

use tokio::signal;
#[cfg(unix)]
use tracing::error;
use tracing::info;

use crate::{
    agents::KnowledgeGraphAgent,
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

    let orch = Orchestrator::new(cfg.clone(), db.clone(), kg, vault);

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
