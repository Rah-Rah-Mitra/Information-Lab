//! Glue: watcher → ingest → KG extraction → (optional) research → vault write.
//!
//! Everything here is pure tokio tasks. The agents themselves live in
//! `agents.rs`; this module only coordinates when to call them.

use std::{path::PathBuf, sync::Arc, time::Duration};

use tokio::{sync::mpsc, time};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    agents::{KgOutput, KnowledgeGraphAgent, ResearchStack},
    config::Config,
    db::{Db, UsageKind},
    ingest::{ingest_pdf, IngestOutcome},
    vault::VaultWriter,
};

pub struct Orchestrator {
    cfg: Config,
    db: Db,
    kg: Arc<KnowledgeGraphAgent>,
    research: Option<Arc<ResearchStack>>,
    vault: Arc<VaultWriter>,
}

impl Orchestrator {
    pub fn new(
        cfg: Config,
        db: Db,
        kg: KnowledgeGraphAgent,
        research: Option<ResearchStack>,
        vault: VaultWriter,
    ) -> Self {
        Self {
            cfg,
            db,
            kg: Arc::new(kg),
            research: research.map(Arc::new),
            vault: Arc::new(vault),
        }
    }

    /// Spawn the ingest consumer: each `PathBuf` from the watcher runs through
    /// hash + pdf_oxide + chunk-insert. We process sequentially to keep RAM
    /// low on the Pi.
    pub fn spawn_ingest(&self, mut rx: mpsc::Receiver<PathBuf>) {
        let db = self.db.clone();
        tokio::spawn(async move {
            while let Some(path) = rx.recv().await {
                match ingest_pdf(&db, &path).await {
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
    /// through the KG agent. If research is enabled, fan out per-concept
    /// research calls (bounded) and append their output to the note.
    pub fn spawn_reasoner(&self) {
        let db = self.db.clone();
        let kg = self.kg.clone();
        let research = self.research.clone();
        let vault = self.vault.clone();
        let budget = self.cfg.batch_token_target as i64;

        tokio::spawn(async move {
            let mut ticker = time::interval(Duration::from_secs(30));
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;

                // Drain as long as there's pending work (governor rate-limits us anyway).
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

                    // Assemble a single prompt with source attribution.
                    let mut user = String::with_capacity((total_tokens as usize) * 4);
                    let primary_doc_hash = chunks[0].doc_hash.clone();
                    for c in &chunks {
                        user.push_str(&format!(
                            "\n\n### Source: {} (pages {}-{})\n\n{}",
                            c.doc_hash, c.page_start, c.page_end, c.content
                        ));
                    }

                    match kg.extract(&user).await {
                        Ok(mut out) => {
                            // Optional enrichment pass.
                            if let Some(r) = &research {
                                enrich(&mut out, r, &user).await;
                            }

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

                            if let Err(e) = vault.write_note(&primary_doc_hash, &out).await {
                                error!(error = %e, "vault write failed");
                                let _ = db
                                    .mark_batch_error(&batch_id, &format!("vault: {e}"))
                                    .await;
                                continue;
                            }
                            if let Err(e) = db.mark_batch_done(&batch_id).await {
                                error!(error = %e, "mark_batch_done failed");
                            } else {
                                info!(batch_id, "batch done");
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
}

/// Fire research at up to N top concepts and append briefs to the snippet.
/// Serialized (not parallel) so we respect the 15 RPM shared bucket.
async fn enrich(out: &mut KgOutput, research: &Arc<ResearchStack>, source_hint: &str) {
    const MAX_CONCEPTS: usize = 3;
    let picks: Vec<String> = out.entities.iter().take(MAX_CONCEPTS).cloned().collect();
    if picks.is_empty() {
        return;
    }

    let hint = truncate(source_hint, 2_000);
    let mut appended = String::new();

    for concept in picks {
        match research.research(&concept, &hint).await {
            Ok(brief) => {
                appended.push_str(&format!(
                    "\n\n## Research · [[{concept}]]\n\n{}\n\n**Sources:**\n{}",
                    brief.brief,
                    brief
                        .sources
                        .iter()
                        .map(|u| format!("- {u}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                ));
            }
            Err(e) => {
                warn!(concept = %concept, error = %e, "research failed");
            }
        }
    }

    if !appended.is_empty() {
        out.markdown_snippet.push_str(&appended);
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let mut cut = max;
        while !s.is_char_boundary(cut) && cut > 0 {
            cut -= 1;
        }
        format!("{}…", &s[..cut])
    }
}
