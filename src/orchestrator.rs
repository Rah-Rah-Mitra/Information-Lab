//! Glue: watcher → ingest → KG extraction → vault write.
//!
//! Everything here is pure tokio tasks. The agents themselves live in
//! `agents.rs`; this module only coordinates when to call them.

use std::{path::PathBuf, sync::Arc, time::Duration};

use tokio::{sync::mpsc, time};
use tracing::{error, info, warn, Instrument};
use uuid::Uuid;

use crate::{
    agents::KnowledgeGraphAgent,
    config::Config,
    db::{Db, UsageKind},
    ingest::{ingest_pdf, IngestOutcome},
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
}
