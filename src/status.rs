//! Every N seconds, write SYSTEM_STATUS.md to the vault with queue depth,
//! today's API usage, and per-document progress. Keeps the user informed
//! via Obsidian itself without needing to tail logs.

use std::{fmt::Write, path::PathBuf, time::Duration};

use chrono::Utc;
use tokio::{fs, io::AsyncWriteExt, time};
use tracing::{error, info};

use crate::db::{Db, DocumentProgress};
use crate::error::AppResult;

pub fn spawn(db: Db, vault_dir: PathBuf, interval: Duration) {
    tokio::spawn(async move {
        let mut ticker = time::interval(interval);
        ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            if let Err(e) = write_status(&db, &vault_dir).await {
                error!(error = %e, "status write failed");
            }
        }
    });
}

async fn write_status(db: &Db, vault_dir: &PathBuf) -> AppResult<()> {
    let pending = db.pending_count().await?;
    let today = Utc::now().date_naive();
    let usage = db.usage_for(today).await?;
    let progress = db.list_document_progress().await?;

    let mut body = String::new();
    writeln!(body, "# System Status").ok();
    writeln!(body).ok();
    writeln!(body, "_Updated: {}_", Utc::now().to_rfc3339()).ok();
    writeln!(body).ok();
    writeln!(body, "## Queue").ok();
    writeln!(body).ok();
    writeln!(body, "- Pending chunks: **{pending}**").ok();
    writeln!(body).ok();

    writeln!(body, "## API Usage — {}", usage.day).ok();
    writeln!(body).ok();
    writeln!(body, "| Metric | Value |").ok();
    writeln!(body, "| --- | --- |").ok();
    writeln!(body, "| Reasoner calls | {} |", usage.reasoner_calls).ok();
    writeln!(body, "| Vision calls | {} |", usage.vision_calls).ok();
    writeln!(body, "| Tokens sent | {} |", usage.tokens_sent).ok();
    writeln!(body, "| Tokens received | {} |", usage.tokens_received).ok();
    writeln!(body).ok();

    writeln!(body, "## Documents").ok();
    writeln!(body).ok();
    if progress.is_empty() {
        writeln!(body, "_No documents ingested yet._").ok();
    } else {
        writeln!(
            body,
            "| Source | Progress | Done/Total | Pending | Batched | Errors | Status |"
        )
        .ok();
        writeln!(
            body,
            "| --- | --- | --- | --- | --- | --- | --- |"
        )
        .ok();
        for p in &progress {
            writeln!(body, "{}", render_progress_row(p)).ok();
        }
    }

    fs::create_dir_all(vault_dir).await?;
    let path = vault_dir.join("SYSTEM_STATUS.md");
    let mut f = fs::File::create(&path).await?;
    f.write_all(body.as_bytes()).await?;
    f.sync_all().await?;
    info!(path = %path.display(), "status updated");
    Ok(())
}

fn render_progress_row(p: &DocumentProgress) -> String {
    let source = p
        .source_name
        .as_deref()
        .unwrap_or_else(|| p.hash.get(..12).unwrap_or(&p.hash));
    let pct = if p.chunks_total > 0 {
        (p.chunks_done as f64 / p.chunks_total as f64) * 100.0
    } else {
        0.0
    };
    let bar = render_bar(pct);
    let status = if p.completed_at.is_some() {
        "complete"
    } else if p.chunks_batched > 0 {
        "processing"
    } else if p.chunks_pending > 0 {
        "queued"
    } else if p.chunks_error > 0 {
        "errors"
    } else {
        "ingested"
    };
    format!(
        "| {source} | `{bar}` {pct:>5.1}% | {done}/{total} | {pending} | {batched} | {errors} | {status} |",
        source = source,
        bar = bar,
        pct = pct,
        done = p.chunks_done,
        total = p.chunks_total,
        pending = p.chunks_pending,
        batched = p.chunks_batched,
        errors = p.chunks_error,
        status = status,
    )
}

fn render_bar(pct: f64) -> String {
    const WIDTH: usize = 10;
    let filled = ((pct / 100.0) * WIDTH as f64).round().clamp(0.0, WIDTH as f64) as usize;
    let empty = WIDTH - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
