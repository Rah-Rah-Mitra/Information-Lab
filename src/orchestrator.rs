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
        curator::{Formula, NoteRef, TopicCuratorAgent},
        derivation::DerivationChainAgent,
        formula_extractor::FormulaExtractorAgent,
        harvester::FormulaHarvesterAgent,
        report::{ReportInput, ReportWriterAgent},
        research_request::{ResearchContext, ResearchRequestAgent},
        retrier::ErrorRetrierAgent,
        theorem::{TheoremInput, TheoremProverAgent},
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
    pub fn new(cfg: Config, db: Db, kg: KnowledgeGraphAgent, vault: VaultWriter) -> Self {
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
        let tau = self.cfg.formula_detect_tau;
        tokio::spawn(async move {
            while let Some(path) = rx.recv().await {
                let span = tracing::info_span!("ingest", path = %path.display());
                match ingest_pdf(&db, &watch_dir, &path, tau)
                    .instrument(span)
                    .await
                {
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
            time::sleep(Duration::from_secs(1)).await;
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
                                let _ =
                                    db.mark_batch_error(&batch_id, &format!("vault: {e}")).await;
                                continue;
                            }

                            // Capture doc_hashes touched by this batch BEFORE
                            // mark_batch_done flips state to 'done', so we can
                            // check per-document completion afterwards.
                            let touched_docs =
                                db.batch_doc_hashes(&batch_id).await.unwrap_or_default();

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
    pub fn spawn_research(&self, curator: TopicCuratorAgent, bridge: BridgeFinderAgent) {
        let db = self.db.clone();
        let vault = self.vault.clone();
        let interval = self.cfg.research_interval;
        let tau = self.cfg.bridge_confidence_tau;
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(5)).await;
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
            time::sleep(Duration::from_secs(15)).await;
            let mut ticker = time::interval(Duration::from_secs(300));
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                // Drain any queued Harvest tasks (scheduler enqueues them).
                let batch_id = Uuid::new_v4().to_string();
                let claimed = db.claim_agent_task(AgentTaskKind::Harvest, &batch_id).await;
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
                                            .map(|r| crate::agents::curator::Formula {
                                                latex: r.latex,
                                                symbols: r
                                                    .symbols
                                                    .unwrap_or_default()
                                                    .split(',')
                                                    .filter(|s| !s.is_empty())
                                                    .map(|s| s.to_string())
                                                    .collect(),
                                                context_caption: r.context.unwrap_or_default(),
                                                note_rel_path: r.note_rel_path,
                                                derived: false,
                                            })
                                            .collect::<Vec<_>>();
                                        if let Err(e) = vault.write_formulas_index(&formulas).await
                                        {
                                            warn!(error = %e, "formulas index write failed");
                                        }
                                        if let Err(e) =
                                            db.increment_usage(UsageKind::Harvester, 0, 0).await
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

    /// Error-chunk retrier tick. Pure DB work — no LLM call — so this runs
    /// on its own cadence independent of the reasoner/research ticks.
    /// Requeued chunks reappear as `'pending'` and the reasoner loop picks
    /// them up on its next batch claim.
    pub fn spawn_error_retrier(&self, retrier: ErrorRetrierAgent) {
        let interval = self.cfg.error_retry_interval;
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(25)).await;
            let mut ticker = time::interval(interval);
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                match retrier.sweep().await {
                    Ok(out) if out.promoted == 0 && out.requeued == 0 => {}
                    Ok(out) => {
                        info!(
                            promoted = out.promoted,
                            requeued = out.requeued,
                            "error retrier sweep"
                        );
                    }
                    Err(e) => {
                        warn!(error = %e, "error retrier sweep failed");
                    }
                }
            }
        });
    }

    /// Heavy-tier research drain. Every `research_interval` tick, pull one
    /// Theorem + one Derivation + one Report task (each optional) and run
    /// them in parallel. They share the 31B tier's rate budget through
    /// `Limiter::admit(Role::*)` inside each agent.
    pub fn spawn_heavy_research(
        &self,
        theorem: TheoremProverAgent,
        derivation: DerivationChainAgent,
        report: ReportWriterAgent,
    ) {
        let db = self.db.clone();
        let vault = self.vault.clone();
        let interval = self.cfg.research_interval;
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(10)).await;
            let mut ticker = time::interval(interval);
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                let th = drain_theorem(&db, &vault, &theorem);
                let de = drain_derivation(&db, &vault, &derivation);
                let re = drain_report(&db, &vault, &report);
                let (rt, rd, rr) = tokio::join!(th, de, re);
                if let Err(e) = rt {
                    warn!(error = %e, "theorem drain failed");
                }
                if let Err(e) = rd {
                    warn!(error = %e, "derivation drain failed");
                }
                if let Err(e) = rr {
                    warn!(error = %e, "report drain failed");
                }
            }
        });
    }

    /// Light-tier formula extractor drain. Each tick pulls one
    /// `FormulaExtract` task (enqueued by `ingest.rs` when the math-density
    /// heuristic flagged a chunk), calls the LLM, and upserts every
    /// salvaged formula into the shared `formulas` corpus. The downstream
    /// harvester/curator then see them like any regex-discovered formula.
    pub fn spawn_formula_extractor(&self, agent: FormulaExtractorAgent) {
        let db = self.db.clone();
        let interval = self.cfg.research_interval;
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(20)).await;
            let mut ticker = time::interval(interval);
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                if let Err(e) = drain_formula_extract(&db, &agent).await {
                    warn!(error = %e, "formula_extract drain failed");
                }
            }
        });
    }

    /// API ad-hoc research drain. Pulls `ResearchRequest` tasks and executes
    /// an isolated loop that only uses vault/topic/formula context.
    pub fn spawn_research_requests(&self, agent: ResearchRequestAgent) {
        let db = self.db.clone();
        let vault = self.vault.clone();
        let interval = self.cfg.research_interval;
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(8)).await;
            let mut ticker = time::interval(interval);
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                if let Err(e) = drain_research_request(&db, &vault, &agent).await {
                    warn!(error = %e, "research_request drain failed");
                }
            }
        });
    }

    /// Idle scheduler tick. Enqueues Curate / Bridge / Harvest tasks based
    /// on vault deltas and DB snapshots.
    pub fn spawn_idle_scheduler(&self, scheduler: Scheduler) {
        let interval = self.cfg.scheduler_interval;
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(30)).await;
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
    let Some(task) = db
        .claim_agent_task(AgentTaskKind::Curate, &batch_id)
        .await?
    else {
        return Ok(());
    };
    let payload: serde_json::Value =
        serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);
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
    let Some(task) = db
        .claim_agent_task(AgentTaskKind::Bridge, &batch_id)
        .await?
    else {
        return Ok(());
    };
    let payload: serde_json::Value =
        serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);
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

async fn drain_theorem(
    db: &Db,
    vault: &Arc<VaultWriter>,
    theorem: &TheoremProverAgent,
) -> crate::error::AppResult<()> {
    let batch_id = Uuid::new_v4().to_string();
    let Some(task) = db
        .claim_agent_task(AgentTaskKind::Theorem, &batch_id)
        .await?
    else {
        return Ok(());
    };
    info!(
        target: "agent.spawn",
        role = "theorem", tier = "heavy", task_id = task.id, "theorem claimed"
    );
    let payload: serde_json::Value =
        serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);
    let s = |k: &str| {
        payload
            .get(k)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    let topic_a = s("topic_a");
    let topic_b = s("topic_b");
    let bridge_rel_path = s("bridge_rel_path");
    if topic_a.is_empty() || topic_b.is_empty() || bridge_rel_path.is_empty() {
        db.fail_agent_task(task.id, "theorem: missing payload fields")
            .await?;
        return Ok(());
    }

    // Hypothesis and summaries: pull from the bridge note body if available.
    let vault_dir = vault.vault_dir().to_path_buf();
    let bridge_abs = vault_dir.join(&bridge_rel_path);
    let bridge_body = tokio::fs::read_to_string(&bridge_abs)
        .await
        .unwrap_or_default();
    let hypothesis = parse_yaml_string(&bridge_body, "summary").unwrap_or_default();

    let summary_a = summarise_topic_by_tag(vault, &topic_a).await;
    let summary_b = summarise_topic_by_tag(vault, &topic_b).await;

    let formulas = filter_formulas_for_topics(db, &[&topic_a, &topic_b], vault).await;

    let input = TheoremInput {
        topic_a: topic_a.clone(),
        topic_b: topic_b.clone(),
        bridge_rel_path: bridge_rel_path.clone(),
        bridge_hypothesis: hypothesis,
        summary_a,
        summary_b,
        formulas,
    };

    match theorem.prove(&input).await {
        Ok(note) => {
            if let Err(e) = vault.write_theorem(&note).await {
                error!(error = %e, "theorem vault write failed");
                db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
            } else {
                db.finish_agent_task(task.id).await?;
            }
        }
        Err(e) => {
            error!(error = %e, "theorem failed");
            db.fail_agent_task(task.id, &e.to_string()).await?;
        }
    }
    Ok(())
}

async fn drain_derivation(
    db: &Db,
    vault: &Arc<VaultWriter>,
    derivation: &DerivationChainAgent,
) -> crate::error::AppResult<()> {
    let batch_id = Uuid::new_v4().to_string();
    let Some(task) = db
        .claim_agent_task(AgentTaskKind::Derivation, &batch_id)
        .await?
    else {
        return Ok(());
    };
    info!(
        target: "agent.spawn",
        role = "derivation", tier = "heavy", task_id = task.id, "derivation claimed"
    );
    let payload: serde_json::Value =
        serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);
    let seed_topic = payload
        .get("seed_topic")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let notes: Vec<String> = payload
        .get("notes")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    if seed_topic.is_empty() || notes.is_empty() {
        db.fail_agent_task(task.id, "derivation: missing payload fields")
            .await?;
        return Ok(());
    }
    let all_formulas = db.list_formulas().await?;
    let formulas: Vec<Formula> = all_formulas
        .into_iter()
        .filter(|f| notes.iter().any(|n| n == &f.note_rel_path))
        .map(|r| Formula {
            latex: r.latex,
            symbols: r
                .symbols
                .unwrap_or_default()
                .split(',')
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect(),
            context_caption: r.context.unwrap_or_default(),
            note_rel_path: r.note_rel_path,
            derived: false,
        })
        .collect();
    if formulas.is_empty() {
        db.fail_agent_task(task.id, "derivation: no formulas resolvable")
            .await?;
        return Ok(());
    }

    match derivation.chain(&seed_topic, &formulas).await {
        Ok(chain) => {
            if let Err(e) = vault.write_derivation(&chain).await {
                error!(error = %e, "derivation vault write failed");
                db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
            } else {
                db.finish_agent_task(task.id).await?;
            }
        }
        Err(e) => {
            error!(error = %e, "derivation failed");
            db.fail_agent_task(task.id, &e.to_string()).await?;
        }
    }
    Ok(())
}

async fn drain_report(
    db: &Db,
    vault: &Arc<VaultWriter>,
    report: &ReportWriterAgent,
) -> crate::error::AppResult<()> {
    let batch_id = Uuid::new_v4().to_string();
    let Some(task) = db
        .claim_agent_task(AgentTaskKind::Report, &batch_id)
        .await?
    else {
        return Ok(());
    };
    info!(
        target: "agent.spawn",
        role = "report", tier = "heavy", task_id = task.id, "report claimed"
    );
    let payload: serde_json::Value =
        serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);
    let date = payload
        .get("date")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    if date.is_empty() {
        db.fail_agent_task(task.id, "report: missing date").await?;
        return Ok(());
    }

    // Gather the last 24h of Syntheses / Bridges / Theorems from vault.
    let inputs = gather_report_inputs(vault).await;
    if inputs.is_empty() {
        // Nothing produced in the window — finish silently without an LLM call.
        db.finish_agent_task(task.id).await?;
        return Ok(());
    }

    match report.write(&date, &inputs).await {
        Ok(r) => {
            if let Err(e) = vault.write_report(&r).await {
                error!(error = %e, "report vault write failed");
                db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
            } else {
                db.finish_agent_task(task.id).await?;
            }
        }
        Err(e) => {
            error!(error = %e, "report failed");
            db.fail_agent_task(task.id, &e.to_string()).await?;
        }
    }
    Ok(())
}

async fn drain_formula_extract(
    db: &Db,
    agent: &FormulaExtractorAgent,
) -> crate::error::AppResult<()> {
    let batch_id = Uuid::new_v4().to_string();
    let Some(task) = db
        .claim_agent_task(AgentTaskKind::FormulaExtract, &batch_id)
        .await?
    else {
        return Ok(());
    };
    info!(
        target: "agent.spawn",
        role = "formula_extractor",
        tier = "light",
        task_id = task.id,
        "formula_extract claimed"
    );
    let payload: serde_json::Value =
        serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);
    let chunk_id = payload
        .get("chunk_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let source_name = payload
        .get("source_name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    if chunk_id == 0 {
        db.fail_agent_task(task.id, "formula_extract: missing chunk_id")
            .await?;
        return Ok(());
    }
    let Some(content) = db.get_chunk_content(chunk_id).await? else {
        db.fail_agent_task(task.id, "formula_extract: chunk not found")
            .await?;
        return Ok(());
    };

    match agent.extract(&content).await {
        Ok(out) => {
            let note_rel_path = format!("_FormulaChunks/{source_name}/{chunk_id}.md");
            let mut inserted_n = 0_u32;
            for f in out.formulas {
                let latex = f.latex.trim().to_string();
                if latex.is_empty() {
                    continue;
                }
                let mut syms = f.symbols.clone();
                syms.sort();
                syms.dedup();
                let latex_norm = syms.join("|");
                let symbols_csv = syms.join(",");
                match db
                    .upsert_formula(
                        &latex_norm,
                        &latex,
                        &symbols_csv,
                        &f.context_caption,
                        &note_rel_path,
                    )
                    .await
                {
                    Ok(true) => inserted_n += 1,
                    Ok(false) => {}
                    Err(e) => warn!(error = %e, "upsert_formula failed"),
                }
            }
            let _ = db.increment_usage(UsageKind::Harvester, 0, 0).await;
            info!(chunk_id, new_formulas = inserted_n, "formula_extract done");
            db.finish_agent_task(task.id).await?;
        }
        Err(e) => {
            error!(error = %e, chunk_id, "formula_extract failed");
            db.fail_agent_task(task.id, &e.to_string()).await?;
        }
    }
    Ok(())
}

async fn drain_research_request(
    db: &Db,
    vault: &Arc<VaultWriter>,
    agent: &ResearchRequestAgent,
) -> crate::error::AppResult<()> {
    let batch_id = Uuid::new_v4().to_string();
    let Some(task) = db
        .claim_agent_task(AgentTaskKind::ResearchRequest, &batch_id)
        .await?
    else {
        return Ok(());
    };

    let payload: serde_json::Value =
        serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);
    let problem = payload
        .get("problem")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .trim()
        .to_string();
    let max_iterations = payload
        .get("max_iterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(2)
        .clamp(1, 8) as u8;
    let skills_scope: Vec<String> = payload
        .get("skills_scope")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    if problem.is_empty() {
        db.fail_agent_task(task.id, "research request: missing problem")
            .await?;
        return Ok(());
    }

    let scope = if skills_scope.is_empty() {
        vec![problem.clone()]
    } else {
        skills_scope.clone()
    };

    let mut topic_context = Vec::new();
    for topic in &scope {
        let s = summarise_topic_by_tag(vault, topic).await;
        if !s.trim().is_empty() {
            topic_context.push(format!("## {topic}\n{s}"));
        }
    }
    let topic_refs: Vec<&str> = scope.iter().map(String::as_str).collect();
    let formula_context: Vec<String> = filter_formulas_for_topics(db, &topic_refs, vault)
        .await
        .into_iter()
        .take(24)
        .map(|f| format!("{} — {}", f.latex, f.context_caption))
        .collect();

    let ctx = ResearchContext {
        problem: problem.clone(),
        max_iterations,
        skills_scope,
        topic_context,
        formula_context,
    };

    match agent.run(&ctx).await {
        Ok(result) => {
            let dir = vault
                .vault_dir()
                .join("Generated")
                .join("_ResearchRequests");
            tokio::fs::create_dir_all(&dir).await?;
            let ts = chrono::Utc::now().format("%Y%m%d-%H%M%S");
            let filename = format!("research-request-{}-{ts}.md", task.id);
            let abs = dir.join(filename);
            let body = format!(
                "---\nkind: research-request\ntask_id: {}\nproblem: {}\nconfidence: {:.2}\n---\n\n# {}\n\n*{}*\n\n{}",
                task.id,
                problem.replace('\n', " "),
                result.confidence,
                result.title,
                result.summary,
                result.markdown_body
            );
            tokio::fs::write(abs, body).await?;
            db.finish_agent_task(task.id).await?;
        }
        Err(e) => {
            db.fail_agent_task(task.id, &e.to_string()).await?;
        }
    }

    Ok(())
}

async fn summarise_topic_by_tag(vault: &Arc<VaultWriter>, tag: &str) -> String {
    let abs = vault.vault_dir().join("Topics").join(format!("{tag}.md"));
    tokio::fs::read_to_string(&abs).await.unwrap_or_default()
}

async fn filter_formulas_for_topics(
    db: &Db,
    topic_tags: &[&str],
    vault: &Arc<VaultWriter>,
) -> Vec<Formula> {
    let mut allowed: std::collections::HashSet<String> = Default::default();
    let re = regex::Regex::new(r"\(([^)]+\.md)\)").unwrap();
    for tag in topic_tags {
        let topic_file = vault.vault_dir().join("Topics").join(format!("{tag}.md"));
        let content = tokio::fs::read_to_string(&topic_file)
            .await
            .unwrap_or_default();
        for caps in re.captures_iter(&content) {
            allowed.insert(caps.get(1).unwrap().as_str().to_string());
        }
    }
    match db.list_formulas().await {
        Ok(rows) => rows
            .into_iter()
            .filter(|r| allowed.contains(&r.note_rel_path))
            .map(|r| Formula {
                latex: r.latex,
                symbols: r
                    .symbols
                    .unwrap_or_default()
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect(),
                context_caption: r.context.unwrap_or_default(),
                note_rel_path: r.note_rel_path,
                derived: false,
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

async fn gather_report_inputs(vault: &Arc<VaultWriter>) -> Vec<ReportInput> {
    let root = vault.vault_dir().join("Generated");
    let kinds = [
        ("synthesis", "_Syntheses"),
        ("bridge", "_Bridges"),
        ("theorem", "_Theorems"),
    ];
    let cutoff = std::time::SystemTime::now()
        .checked_sub(Duration::from_secs(86_400))
        .unwrap_or(std::time::UNIX_EPOCH);
    let mut out = Vec::new();
    for (kind, dir_name) in kinds {
        let dir = root.join(dir_name);
        let Ok(mut entries) = tokio::fs::read_dir(&dir).await else {
            continue;
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let md = match entry.metadata().await {
                Ok(m) => m,
                Err(_) => continue,
            };
            let modified = md.modified().unwrap_or(std::time::UNIX_EPOCH);
            if modified < cutoff {
                continue;
            }
            let content = tokio::fs::read_to_string(&path).await.unwrap_or_default();
            let title = parse_yaml_string(&content, "title").unwrap_or_default();
            let summary = parse_yaml_string(&content, "summary").unwrap_or_default();
            let rel = path
                .strip_prefix(vault.vault_dir())
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.to_string_lossy().to_string());
            out.push(ReportInput {
                kind: kind.to_string(),
                title,
                summary,
                note_rel_path: rel,
            });
        }
    }
    out
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
            if in_fm {
                return None;
            }
            in_fm = true;
            continue;
        }
        if !in_fm {
            continue;
        }
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
            if !in_fm {
                in_fm = true;
                continue;
            }
            if !seen_fm_close {
                seen_fm_close = true;
                continue;
            }
        }
        if seen_fm_close {
            out.push_str(line);
            out.push('\n');
        }
    }
    if out.len() > 4000 {
        let mut cut = 4000;
        while !out.is_char_boundary(cut) && cut > 0 {
            cut -= 1;
        }
        out.truncate(cut);
        out.push_str("\n…");
    }
    out
}
