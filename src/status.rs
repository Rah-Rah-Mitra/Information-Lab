//! Every N seconds, write SYSTEM_STATUS.md to the vault with queue depth,
//! today's API usage, and per-document progress. Keeps the user informed
//! via Obsidian itself without needing to tail logs.

use std::{fmt::Write, path::PathBuf, time::Duration};

use chrono::Utc;
use tokio::{fs, io::AsyncWriteExt, time};
use tracing::{error, info};

use crate::db::{AgentEventRow, Db, DocumentProgress};
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
    let events = db.list_recent_agent_events(20).await.unwrap_or_default();

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
    writeln!(body, "| Reasoner (extractor) calls | {} |", usage.reasoner_calls).ok();
    writeln!(body, "| Vision calls | {} |", usage.vision_calls).ok();
    writeln!(body, "| Curator calls | {} |", usage.curator_calls).ok();
    writeln!(body, "| Bridge calls | {} |", usage.bridge_calls).ok();
    writeln!(body, "| Harvester calls | {} |", usage.harvester_calls).ok();
    writeln!(body, "| Search (Tavily) calls | {} |", usage.search_calls).ok();
    writeln!(body, "| Theorem calls | {} |", usage.theorem_calls).ok();
    writeln!(body, "| Derivation calls | {} |", usage.derivation_calls).ok();
    writeln!(body, "| Report calls | {} |", usage.report_calls).ok();
    writeln!(body, "| Formula-extract calls | {} |", usage.formula_extract_calls).ok();
    writeln!(body, "| Tokens sent (est.) | {} |", usage.tokens_sent).ok();
    writeln!(body, "| Tokens received (est.) | {} |", usage.tokens_received).ok();
    writeln!(body).ok();
    writeln!(
        body,
        "_Token counts are chars/4 estimates — the autoagents 0.3.7 Google backend does not surface `usageMetadata`._",
    )
    .ok();
    writeln!(body).ok();

    writeln!(body, "## Documents").ok();
    writeln!(body).ok();
    if progress.is_empty() {
        writeln!(body, "_No documents ingested yet._").ok();
    } else {
        writeln!(
            body,
            "| Source | Path | Progress | Done/Total | Pending | Batched | Errors | Status |"
        )
        .ok();
        writeln!(
            body,
            "| --- | --- | --- | --- | --- | --- | --- | --- |"
        )
        .ok();
        for p in &progress {
            writeln!(body, "{}", render_progress_row(p)).ok();
        }
    }
    writeln!(body).ok();

    writeln!(body, "## Recent agent activity").ok();
    writeln!(body).ok();
    if events.is_empty() {
        writeln!(body, "_No agent events recorded yet._").ok();
    } else {
        writeln!(
            body,
            "| ts | agent | kind | tokens (s/r) | dur (ms) | input → output |"
        )
        .ok();
        writeln!(body, "| --- | --- | --- | --- | --- | --- |").ok();
        for e in &events {
            writeln!(body, "{}", render_event_row(e)).ok();
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
        "| {source} | `{path}` | `{bar}` {pct:>5.1}% | {done}/{total} | {pending} | {batched} | {errors} | {status} |",
        source = source,
        path = cell_escape(&p.path),
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

fn render_event_row(e: &AgentEventRow) -> String {
    let ts = e.ts.get(..19).unwrap_or(&e.ts);
    let input = e.input_summary.as_deref().unwrap_or("");
    let output = e.output_summary.as_deref().unwrap_or("");
    let dur = e
        .duration_ms
        .map(|d| d.to_string())
        .unwrap_or_else(|| "—".into());
    let io = format!(
        "{} → {}",
        cell_escape(&truncate(input, 80)),
        cell_escape(&truncate(output, 80)),
    );
    format!(
        "| {ts} | {role} | {kind} | {ts_s}/{ts_r} | {dur} | {io} |",
        ts = ts,
        role = e.agent_role,
        kind = e.event_kind,
        ts_s = e.tokens_sent,
        ts_r = e.tokens_received,
        dur = dur,
        io = io,
    )
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let cut: String = s.chars().take(max).collect();
    format!("{cut}…")
}

fn cell_escape(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

fn render_bar(pct: f64) -> String {
    const WIDTH: usize = 10;
    let filled = ((pct / 100.0) * WIDTH as f64).round().clamp(0.0, WIDTH as f64) as usize;
    let empty = WIDTH - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
