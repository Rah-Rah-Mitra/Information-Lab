//! Glue: watcher → ingest → KG extraction → vault write.
//!
//! Everything here is pure tokio tasks. The agents themselves live in
//! `agents.rs`; this module only coordinates when to call them.

use std::{path::PathBuf, sync::Arc, time::Duration};

use tokio::{sync::mpsc, time};
use tracing::{error, info, warn, Instrument};
use uuid::Uuid;

use crate::{
    agents::{
        bridge::{BridgeFinderAgent, TopicPair},
        curator::{NoteRef, TopicCuratorAgent},
        harvester::FormulaHarvesterAgent,
        KnowledgeGraphAgent,
    },
    config::Config,
    db::{AgentTaskKind, Db, UsageKind},
    ingest::{ingest_pdf, IngestOutcome},
    scheduler::Scheduler,
    vault::VaultWriter,
};

pub struct Orchestrator {
    cfg: Config,
    db: Db,
    kg: Arc<KnowledgeGraphAgent>,
    vault: Arc<VaultWriter>,
}

impl Orchestrator {
    pub fn new(
        cfg: Config,
        db: Db,
        kg: KnowledgeGraphAgent,
        vault: VaultWriter,
    ) -> Self {
        Self {
            cfg,
            db,
            kg: Arc::new(kg),
            vault: Arc::new(vault),
        }
    }

    /// Spawn the ingest consumer: each `PathBuf` from the watcher runs through
    /// hash + pdf_oxide + chunk-insert. We process sequentially to keep RAM
    /// low on the Pi.
    pub fn spawn_ingest(&self, mut rx: mpsc::Receiver<PathBuf>) {
        let db = self.db.clone();
        let watch_dir = self.cfg.watch_dir.clone();
        tokio::spawn(async move {
            while let Some(path) = rx.recv().await {
                let span = tracing::info_span!("ingest", path = %path.display());
                match ingest_pdf(&db, &watch_dir, &path).instrument(span).await {
                    Ok(IngestOutcome::Ingested { hash, chunks }) => {
                        info!(%hash, chunks, "ingested");
                    }
                    Ok(IngestOutcome::Duplicate) => {}
                    Ok(IngestOutcome::Empty) => {
                        warn!(path = %path.display(), "empty pdf");
                    }
                    Err(e) => {
                        error!(error = %e, path = %path.display(), "ingest failed");
                    }
                }
            }
        });
    }

    /// Spawn the batcher: every few seconds, try to claim a batch and push it
    /// through the KG agent, then write the resulting note to the vault.
    pub fn spawn_reasoner(&self) {
        let db = self.db.clone();
        let kg = self.kg.clone();
        let vault = self.vault.clone();
        let budget = self.cfg.batch_token_target as i64;

        tokio::spawn(async move {
            let mut ticker = time::interval(Duration::from_secs(30));
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;

                loop {
                    let batch_id = Uuid::new_v4().to_string();
                    let chunks = match db.claim_batch(budget, &batch_id).await {
                        Ok(c) => c,
                        Err(e) => {
                            error!(error = %e, "claim_batch failed");
                            break;
                        }
                    };
                    if chunks.is_empty() {
                        break;
                    }

                    let total_tokens: i64 = chunks.iter().map(|c| c.token_estimate).sum();
                    info!(
                        batch_id,
                        count = chunks.len(),
                        tokens = total_tokens,
                        "batch claimed"
                    );

                    let mut user = String::with_capacity((total_tokens as usize) * 4);
                    let primary_doc_hash = chunks[0].doc_hash.clone();
                    let source_name = db
                        .document_source_name(&primary_doc_hash)
                        .await
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| primary_doc_hash.clone());
                    for c in &chunks {
                        user.push_str(&format!(
                            "\n\n### Source: {} (pages {}-{})\n\n{}",
                            c.doc_hash, c.page_start, c.page_end, c.content
                        ));
                    }

                    match kg.extract(&user).await {
                        Ok(out) => {
                            if let Err(e) = db
                                .increment_usage(
                                    UsageKind::Reasoner,
                                    out.tokens_sent,
                                    out.tokens_received,
                                )
                                .await
                            {
                                warn!(error = %e, "usage increment failed");
                            }

                            if let Err(e) = vault.write_note(&source_name, &out).await {
                                error!(error = %e, "vault write failed");
                                let _ = db
                                    .mark_batch_error(&batch_id, &format!("vault: {e}"))
                                    .await;
                                continue;
                            }

                            // Capture doc_hashes touched by this batch BEFORE
                            // mark_batch_done flips state to 'done', so we can
                            // check per-document completion afterwards.
                            let touched_docs = db
                                .batch_doc_hashes(&batch_id)
                                .await
                                .unwrap_or_default();

                            if let Err(e) = db.mark_batch_done(&batch_id).await {
                                error!(error = %e, "mark_batch_done failed");
                            } else {
                                info!(batch_id, "batch done");
                                for dh in &touched_docs {
                                    match db.maybe_mark_document_complete(dh).await {
                                        Ok(true) => info!(
                                            doc_hash = %dh,
                                            source = %source_name,
                                            "document complete"
                                        ),
                                        Ok(false) => {}
                                        Err(e) => warn!(
                                            error = %e,
                                            doc_hash = %dh,
                                            "completion check failed"
                                        ),
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!(error = %e, batch_id, "kg extraction failed");
                            let _ = db.mark_batch_error(&batch_id, &e.to_string()).await;
                        }
                    }
                }
            }
        });
    }

    /// Research tick: every `cfg.research_interval`, drain up to one
    /// Curate and one Bridge task concurrently (Parallel workflow shape).
    pub fn spawn_research(
        &self,
        curator: TopicCuratorAgent,
        bridge: BridgeFinderAgent,
    ) {
        let db = self.db.clone();
        let vault = self.vault.clone();
        let interval = self.cfg.research_interval;
        let tau = self.cfg.bridge_confidence_tau;
        tokio::spawn(async move {
            let mut ticker = time::interval(interval);
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                let curate_fut = drain_curate(&db, &vault, &curator);
                let bridge_fut = drain_bridge(&db, &vault, &bridge, tau);
                let (rc, rb) = tokio::join!(curate_fut, bridge_fut);
                if let Err(e) = rc {
                    warn!(error = %e, "curate drain failed");
                }
                if let Err(e) = rb {
                    warn!(error = %e, "bridge drain failed");
                }
            }
        });
    }

    /// Formula harvest tick. Runs on a longer cadence than research —
    /// regex-only, so cheap, but RAM-sensitive on the Pi.
    pub fn spawn_harvester(&self, harvester: FormulaHarvesterAgent) {
        let db = self.db.clone();
        let vault = self.vault.clone();
        tokio::spawn(async move {
            let mut ticker = time::interval(Duration::from_secs(300));
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                // Drain any queued Harvest tasks (scheduler enqueues them).
                let batch_id = Uuid::new_v4().to_string();
                let claimed = db
                    .claim_agent_task(AgentTaskKind::Harvest, &batch_id)
                    .await;
                match claimed {
                    Ok(Some(task)) => {
                        match harvester.harvest(64).await {
                            Ok(res) => {
                                info!(
                                    new = res.new_formulas.len(),
                                    scanned = res.notes_scanned,
                                    "harvest ok"
                                );
                                // Refresh the rendered formulas index.
                                match db.list_formulas().await {
                                    Ok(rows) => {
                                        let formulas = rows
                                            .into_iter()
                                            .map(|r| {
                                                crate::agents::curator::Formula {
                                                    latex: r.latex,
                                                    symbols: r
                                                        .symbols
                                                        .unwrap_or_default()
                                                        .split(',')
                                                        .filter(|s| !s.is_empty())
                                                        .map(|s| s.to_string())
                                                        .collect(),
                                                    context_caption: r
                                                        .context
                                                        .unwrap_or_default(),
                                                    note_rel_path: r.note_rel_path,
                                                    derived: false,
                                                }
                                            })
                                            .collect::<Vec<_>>();
                                        if let Err(e) =
                                            vault.write_formulas_index(&formulas).await
                                        {
                                            warn!(error = %e, "formulas index write failed");
                                        }
                                        if let Err(e) = db
                                            .increment_usage(
                                                UsageKind::Harvester,
                                                0,
                                                0,
                                            )
                                            .await
                                        {
                                            warn!(error = %e, "usage increment failed");
                                        }
                                    }
                                    Err(e) => {
                                        warn!(error = %e, "list_formulas failed");
                                    }
                                }
                                if let Err(e) = db.finish_agent_task(task.id).await {
                                    warn!(error = %e, "finish_agent_task failed");
                                }
                            }
                            Err(e) => {
                                error!(error = %e, "harvest failed");
                                let _ = db.fail_agent_task(task.id, &e.to_string()).await;
                            }
                        }
                    }
                    Ok(None) => {}
                    Err(e) => {
                        warn!(error = %e, "claim_agent_task(Harvest) failed");
                    }
                }
            }
        });
    }

    /// Idle scheduler tick. Enqueues Curate / Bridge / Harvest tasks based
    /// on vault deltas and DB snapshots.
    pub fn spawn_idle_scheduler(&self, scheduler: Scheduler) {
        let interval = self.cfg.scheduler_interval;
        tokio::spawn(async move {
            let mut ticker = time::interval(interval);
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                if let Err(e) = scheduler.tick().await {
                    warn!(error = %e, "scheduler tick failed");
                }
            }
        });
    }
}

// ----------------------------------------------------------------------------
// Research drain helpers — free functions to keep the tokio::join! concise.
// ----------------------------------------------------------------------------

async fn drain_curate(
    db: &Db,
    vault: &Arc<VaultWriter>,
    curator: &TopicCuratorAgent,
) -> crate::error::AppResult<()> {
    let batch_id = Uuid::new_v4().to_string();
    let Some(task) = db.claim_agent_task(AgentTaskKind::Curate, &batch_id).await? else {
        return Ok(());
    };
    let payload: serde_json::Value = serde_json::from_str(&task.payload)
        .unwrap_or(serde_json::Value::Null);
    let topic = payload
        .get("topic")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let note_paths: Vec<String> = payload
        .get("notes")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let vault_dir = vault.vault_dir().to_path_buf();
    let mut notes = Vec::new();
    for rel in note_paths.into_iter().take(12) {
        let abs = vault_dir.join(&rel);
        let content = match tokio::fs::read_to_string(&abs).await {
            Ok(c) => c,
            Err(_) => continue,
        };
        let source = parse_yaml_string(&content, "source").unwrap_or_default();
        let title = parse_yaml_string(&content, "title").unwrap_or_default();
        let summary = parse_yaml_string(&content, "summary").unwrap_or_default();
        let snippet = content_body(&content);
        notes.push(NoteRef {
            note_title: title,
            source,
            summary,
            markdown_snippet: snippet,
            note_rel_path: rel,
        });
    }
    if notes.is_empty() {
        db.fail_agent_task(task.id, "no notes resolvable").await?;
        return Ok(());
    }

    match curator.curate(&topic, &notes).await {
        Ok(syn) => {
            if let Err(e) = vault.write_synthesis(&syn).await {
                error!(error = %e, "synthesis vault write failed");
                db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
            } else {
                let _ = db.increment_usage(UsageKind::Curator, 0, 0).await;
                db.finish_agent_task(task.id).await?;
            }
        }
        Err(e) => {
            error!(error = %e, "curator failed");
            db.fail_agent_task(task.id, &e.to_string()).await?;
        }
    }
    Ok(())
}

async fn drain_bridge(
    db: &Db,
    vault: &Arc<VaultWriter>,
    bridge: &BridgeFinderAgent,
    tau: f32,
) -> crate::error::AppResult<()> {
    let batch_id = Uuid::new_v4().to_string();
    let Some(task) = db.claim_agent_task(AgentTaskKind::Bridge, &batch_id).await? else {
        return Ok(());
    };
    let payload: serde_json::Value = serde_json::from_str(&task.payload)
        .unwrap_or(serde_json::Value::Null);
    let s = |k: &str| {
        payload
            .get(k)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    let pair = TopicPair {
        topic_a: s("topic_a"),
        topic_b: s("topic_b"),
        source_a: s("source_a"),
        source_b: s("source_b"),
        summary_a: summarise_topic(vault, &s("topic_a"), &payload, "notes_a").await,
        summary_b: summarise_topic(vault, &s("topic_b"), &payload, "notes_b").await,
    };

    match bridge.run(pair.clone()).await {
        Ok(Some(note)) => {
            if note.proposal.confidence < tau {
                db.finish_agent_task(task.id).await?;
                return Ok(());
            }
            match vault.write_bridge(&note).await {
                Ok(abs) => {
                    let rel = abs
                        .strip_prefix(vault.vault_dir())
                        .map(|p| p.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|_| abs.to_string_lossy().to_string());
                    let _ = db
                        .insert_bridge(
                            &pair.topic_a,
                            &pair.topic_b,
                            &pair.source_a,
                            &pair.source_b,
                            note.proposal.confidence,
                            note.iterations as i64,
                            &rel,
                        )
                        .await;
                    let _ = db.increment_usage(UsageKind::Bridge, 0, 0).await;
                    db.finish_agent_task(task.id).await?;
                }
                Err(e) => {
                    error!(error = %e, "bridge vault write failed");
                    db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
                }
            }
        }
        Ok(None) => {
            db.finish_agent_task(task.id).await?;
        }
        Err(e) => {
            error!(error = %e, "bridge failed");
            db.fail_agent_task(task.id, &e.to_string()).await?;
        }
    }
    Ok(())
}

async fn summarise_topic(
    vault: &Arc<VaultWriter>,
    topic: &str,
    payload: &serde_json::Value,
    key: &str,
) -> String {
    let notes: Vec<String> = payload
        .get(key)
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let vault_dir = vault.vault_dir().to_path_buf();
    let mut snippets = Vec::new();
    for rel in notes.into_iter().take(4) {
        if let Ok(content) = tokio::fs::read_to_string(vault_dir.join(&rel)).await {
            let summary = parse_yaml_string(&content, "summary").unwrap_or_default();
            snippets.push(format!("- {rel}: {summary}"));
        }
    }
    format!("Topic **{topic}** notes:\n{}", snippets.join("\n"))
}

fn parse_yaml_string(content: &str, key: &str) -> Option<String> {
    let mut in_fm = false;
    for line in content.lines() {
        if line.trim() == "---" {
            if in_fm { return None; }
            in_fm = true;
            continue;
        }
        if !in_fm { continue; }
        if let Some(rest) = line.strip_prefix(&format!("{key}:")) {
            let v = rest.trim().trim_matches('"').to_string();
            return Some(v);
        }
    }
    None
}

fn content_body(content: &str) -> String {
    let mut seen_fm_close = false;
    let mut out = String::new();
    let mut in_fm = false;
    for line in content.lines() {
        if line.trim() == "---" {
            if !in_fm { in_fm = true; continue; }
            if !seen_fm_close { seen_fm_close = true; continue; }
        }
        if seen_fm_close {
            out.push_str(line);
            out.push('\n');
        }
    }
    if out.len() > 4000 {
        let mut cut = 4000;
        while !out.is_char_boundary(cut) && cut > 0 { cut -= 1; }
        out.truncate(cut);
        out.push_str("\n…");
    }
    out
}
