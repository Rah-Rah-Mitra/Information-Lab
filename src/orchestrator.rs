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
        research_request::{ResearchContext, ResearchRequestAgent, ResearchResult},
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

#[derive(Clone, Copy)]
struct ResearchCtx<'a> {
    request_id: &'a str,
    role: &'a str,
}

async fn emit_research_event(
    db: &Db,
    ctx: ResearchCtx<'_>,
    event_kind: &str,
    step_index: i64,
    phase: &str,
    input_summary: Option<&str>,
    output_summary: Option<&str>,
    tool_name: Option<&str>,
    artifact_path: Option<&str>,
) {
    tracing::info!(
        research_request_id = %ctx.request_id,
        agent_role = %ctx.role,
        step_index,
        event_kind,
        phase,
        "research lifecycle event"
    );
    let _ = db
        .insert_agent_event(
            None,
            None,
            None,
            ctx.role,
            event_kind,
            input_summary,
            output_summary,
            None,
            None,
            0,
            0,
            None,
            Some(ctx.request_id),
            Some(step_index),
            Some(phase),
            tool_name,
            None,
            artifact_path,
        )
        .await;
}

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

    /// Ad-hoc research request drain with solvability gating.
    pub fn spawn_research_requests(&self, agent: ResearchRequestAgent) {
        let db = self.db.clone();
        let vault = self.vault.clone();
        let interval = self.cfg.research_interval;
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(12)).await;
            let mut ticker = time::interval(interval);
            ticker.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                if let Err(e) = drain_research_request(&db, &vault, &agent).await {
                    warn!(error = %e, "research request drain failed");
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
    let request_id = task.id.to_string();
    let ctx = ResearchCtx {
        request_id: &request_id,
        role: "curator",
    };
    emit_research_event(
        db,
        ctx,
        "request_received",
        0,
        "queue",
        Some(&task.payload),
        None,
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "plan_created",
        1,
        "planning",
        None,
        Some("curate task parsed"),
        None,
        None,
    )
    .await;
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
        emit_research_event(
            db,
            ctx,
            "failed",
            2,
            "failed",
            None,
            Some("no notes resolvable"),
            None,
            None,
        )
        .await;
        db.fail_agent_task(task.id, "no notes resolvable").await?;
        return Ok(());
    }
    emit_research_event(
        db,
        ctx,
        "skill_context_loaded",
        2,
        "context",
        None,
        Some("notes loaded"),
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "llm_call",
        3,
        "llm_call",
        Some(&topic),
        None,
        None,
        None,
    )
    .await;

    match curator.curate(&topic, &notes).await {
        Ok(syn) => {
            if let Err(e) = vault.write_synthesis(&syn).await {
                error!(error = %e, "synthesis vault write failed");
                emit_research_event(
                    db,
                    ctx,
                    "failed",
                    5,
                    "failed",
                    None,
                    Some(&e.to_string()),
                    None,
                    None,
                )
                .await;
                db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
            } else {
                let _ = db.increment_usage(UsageKind::Curator, 0, 0).await;
                emit_research_event(
                    db,
                    ctx,
                    "finalized",
                    5,
                    "finalize",
                    None,
                    Some("synthesis written"),
                    None,
                    None,
                )
                .await;
                db.finish_agent_task(task.id).await?;
            }
        }
        Err(e) => {
            error!(error = %e, "curator failed");
            emit_research_event(
                db,
                ctx,
                "failed",
                4,
                "failed",
                None,
                Some(&e.to_string()),
                None,
                None,
            )
            .await;
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
    let request_id = task.id.to_string();
    let ctx = ResearchCtx {
        request_id: &request_id,
        role: "bridge",
    };
    emit_research_event(
        db,
        ctx,
        "request_received",
        0,
        "queue",
        Some(&task.payload),
        None,
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "plan_created",
        1,
        "planning",
        None,
        Some("bridge task parsed"),
        None,
        None,
    )
    .await;
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
    emit_research_event(
        db,
        ctx,
        "skill_context_loaded",
        2,
        "context",
        None,
        Some("topic summaries loaded"),
        None,
        None,
    )
    .await;

    match bridge.run(pair.clone()).await {
        Ok(Some(note)) => {
            emit_research_event(
                db,
                ctx,
                "llm_call",
                3,
                "llm_call",
                None,
                Some("bridge proposal generated"),
                None,
                None,
            )
            .await;
            emit_research_event(
                db,
                ctx,
                "tool_call",
                4,
                "tool_call",
                None,
                Some("literature search considered"),
                Some("tavily"),
                None,
            )
            .await;
            emit_research_event(
                db,
                ctx,
                "critique",
                5,
                "critique",
                None,
                Some("bridge critique complete"),
                None,
                None,
            )
            .await;
            if note.proposal.confidence < tau {
                emit_research_event(
                    db,
                    ctx,
                    "finalized",
                    6,
                    "finalize",
                    None,
                    Some("below threshold"),
                    None,
                    None,
                )
                .await;
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
                    emit_research_event(
                        db,
                        ctx,
                        "finalized",
                        6,
                        "finalize",
                        None,
                        Some("bridge written"),
                        None,
                        Some(&rel),
                    )
                    .await;
                    db.finish_agent_task(task.id).await?;
                }
                Err(e) => {
                    error!(error = %e, "bridge vault write failed");
                    emit_research_event(
                        db,
                        ctx,
                        "failed",
                        6,
                        "failed",
                        None,
                        Some(&e.to_string()),
                        None,
                        None,
                    )
                    .await;
                    db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
                }
            }
        }
        Ok(None) => {
            emit_research_event(
                db,
                ctx,
                "finalized",
                6,
                "finalize",
                None,
                Some("bridge rejected"),
                None,
                None,
            )
            .await;
            db.finish_agent_task(task.id).await?;
        }
        Err(e) => {
            error!(error = %e, "bridge failed");
            emit_research_event(
                db,
                ctx,
                "failed",
                6,
                "failed",
                None,
                Some(&e.to_string()),
                None,
                None,
            )
            .await;
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
    let request_id = task.id.to_string();
    let ctx = ResearchCtx {
        request_id: &request_id,
        role: "theorem",
    };
    emit_research_event(
        db,
        ctx,
        "request_received",
        0,
        "queue",
        Some(&task.payload),
        None,
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "plan_created",
        1,
        "planning",
        None,
        Some("theorem task parsed"),
        None,
        None,
    )
    .await;
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
    emit_research_event(
        db,
        ctx,
        "skill_context_loaded",
        2,
        "context",
        None,
        Some("bridge+formula context loaded"),
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "llm_call",
        3,
        "llm_call",
        None,
        Some("theorem prove"),
        None,
        None,
    )
    .await;

    match theorem.prove(&input).await {
        Ok(note) => {
            if let Err(e) = vault.write_theorem(&note).await {
                error!(error = %e, "theorem vault write failed");
                emit_research_event(
                    db,
                    ctx,
                    "failed",
                    4,
                    "failed",
                    None,
                    Some(&e.to_string()),
                    None,
                    None,
                )
                .await;
                db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
            } else {
                emit_research_event(
                    db,
                    ctx,
                    "finalized",
                    4,
                    "finalize",
                    None,
                    Some("theorem written"),
                    None,
                    None,
                )
                .await;
                db.finish_agent_task(task.id).await?;
            }
        }
        Err(e) => {
            error!(error = %e, "theorem failed");
            emit_research_event(
                db,
                ctx,
                "failed",
                4,
                "failed",
                None,
                Some(&e.to_string()),
                None,
                None,
            )
            .await;
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
    let request_id = task.id.to_string();
    let ctx = ResearchCtx {
        request_id: &request_id,
        role: "derivation",
    };
    emit_research_event(
        db,
        ctx,
        "request_received",
        0,
        "queue",
        Some(&task.payload),
        None,
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "plan_created",
        1,
        "planning",
        None,
        Some("derivation task parsed"),
        None,
        None,
    )
    .await;
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
        emit_research_event(
            db,
            ctx,
            "failed",
            2,
            "failed",
            None,
            Some("no formulas resolvable"),
            None,
            None,
        )
        .await;
        db.fail_agent_task(task.id, "derivation: no formulas resolvable")
            .await?;
        return Ok(());
    }
    emit_research_event(
        db,
        ctx,
        "skill_context_loaded",
        2,
        "context",
        None,
        Some("formulas loaded"),
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "llm_call",
        3,
        "llm_call",
        None,
        Some("derivation chain"),
        None,
        None,
    )
    .await;

    match derivation.chain(&seed_topic, &formulas).await {
        Ok(chain) => {
            if let Err(e) = vault.write_derivation(&chain).await {
                error!(error = %e, "derivation vault write failed");
                emit_research_event(
                    db,
                    ctx,
                    "failed",
                    4,
                    "failed",
                    None,
                    Some(&e.to_string()),
                    None,
                    None,
                )
                .await;
                db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
            } else {
                emit_research_event(
                    db,
                    ctx,
                    "finalized",
                    4,
                    "finalize",
                    None,
                    Some("derivation written"),
                    None,
                    None,
                )
                .await;
                db.finish_agent_task(task.id).await?;
            }
        }
        Err(e) => {
            error!(error = %e, "derivation failed");
            emit_research_event(
                db,
                ctx,
                "failed",
                4,
                "failed",
                None,
                Some(&e.to_string()),
                None,
                None,
            )
            .await;
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
    let request_id = task.id.to_string();
    let ctx = ResearchCtx {
        request_id: &request_id,
        role: "report",
    };
    emit_research_event(
        db,
        ctx,
        "request_received",
        0,
        "queue",
        Some(&task.payload),
        None,
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "plan_created",
        1,
        "planning",
        None,
        Some("report task parsed"),
        None,
        None,
    )
    .await;
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
        emit_research_event(
            db,
            ctx,
            "finalized",
            2,
            "finalize",
            None,
            Some("no inputs in time window"),
            None,
            None,
        )
        .await;
        db.finish_agent_task(task.id).await?;
        return Ok(());
    }
    emit_research_event(
        db,
        ctx,
        "skill_context_loaded",
        2,
        "context",
        None,
        Some("report inputs loaded"),
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "llm_call",
        3,
        "llm_call",
        None,
        Some("report write"),
        None,
        None,
    )
    .await;

    match report.write(&date, &inputs).await {
        Ok(r) => {
            if let Err(e) = vault.write_report(&r).await {
                error!(error = %e, "report vault write failed");
                emit_research_event(
                    db,
                    ctx,
                    "failed",
                    4,
                    "failed",
                    None,
                    Some(&e.to_string()),
                    None,
                    None,
                )
                .await;
                db.fail_agent_task(task.id, &format!("vault: {e}")).await?;
            } else {
                emit_research_event(
                    db,
                    ctx,
                    "finalized",
                    4,
                    "finalize",
                    None,
                    Some("report written"),
                    None,
                    None,
                )
                .await;
                db.finish_agent_task(task.id).await?;
            }
        }
        Err(e) => {
            error!(error = %e, "report failed");
            emit_research_event(
                db,
                ctx,
                "failed",
                4,
                "failed",
                None,
                Some(&e.to_string()),
                None,
                None,
            )
            .await;
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
    let request_id = task.id.to_string();
    let ctx = ResearchCtx {
        request_id: &request_id,
        role: "formula_extractor",
    };
    emit_research_event(
        db,
        ctx,
        "request_received",
        0,
        "queue",
        Some(&task.payload),
        None,
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "plan_created",
        1,
        "planning",
        None,
        Some("formula task parsed"),
        None,
        None,
    )
    .await;
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
        emit_research_event(
            db,
            ctx,
            "failed",
            2,
            "failed",
            None,
            Some("chunk not found"),
            None,
            None,
        )
        .await;
        db.fail_agent_task(task.id, "formula_extract: chunk not found")
            .await?;
        return Ok(());
    };
    emit_research_event(
        db,
        ctx,
        "skill_context_loaded",
        2,
        "context",
        None,
        Some("chunk loaded"),
        None,
        None,
    )
    .await;
    emit_research_event(
        db,
        ctx,
        "llm_call",
        3,
        "llm_call",
        None,
        Some("formula extract"),
        None,
        None,
    )
    .await;

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
            emit_research_event(
                db,
                ctx,
                "finalized",
                4,
                "finalize",
                None,
                Some("formulas persisted"),
                None,
                Some(&note_rel_path),
            )
            .await;
            db.finish_agent_task(task.id).await?;
        }
        Err(e) => {
            error!(error = %e, chunk_id, "formula_extract failed");
            emit_research_event(
                db,
                ctx,
                "failed",
                4,
                "failed",
                None,
                Some(&e.to_string()),
                None,
                None,
            )
            .await;
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
        .claim_agent_task(AgentTaskKind::Research, &batch_id)
        .await?
    else {
        return Ok(());
    };
    let payload: serde_json::Value =
        serde_json::from_str(&task.payload).unwrap_or(serde_json::Value::Null);
    let request_id = format!("research-{}", task.id);
    let ctx = ResearchCtx {
        request_id: &request_id,
        role: "research_request",
    };
    emit_research_event(
        db,
        ctx,
        "request_received",
        0,
        "queue",
        Some(&task.payload),
        None,
        Some("api:research_request"),
        None,
    )
    .await;

    let problem = payload
        .get("problem")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .trim()
        .to_string();
    if problem.is_empty() {
        emit_research_event(
            db,
            ctx,
            "failed",
            1,
            "failed",
            None,
            Some("empty problem payload"),
            None,
            None,
        )
        .await;
        db.fail_agent_task(task.id, "empty problem payload").await?;
        return Ok(());
    }
    let max_iterations = payload
        .get("max_iterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(2)
        .clamp(1, 6) as u8;
    let skills_scope: Vec<String> = payload
        .get("skills_scope")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    emit_research_event(
        db,
        ctx,
        "plan_created",
        1,
        "planning",
        Some(&problem),
        Some("solvability gate started"),
        None,
        None,
    )
    .await;

    let tokens = tokenize_problem(&problem);
    let (topic_context, note_hits) = gather_problem_context(vault, &tokens).await;
    let formula_context = gather_formula_context(db, &tokens, 16).await;
    let formula_hits = formula_context.len();
    let note_cov = (note_hits as f32 / 6.0).clamp(0.0, 1.0);
    let formula_cov = (formula_hits as f32 / 6.0).clamp(0.0, 1.0);
    let coverage = 0.75 * note_cov + 0.25 * formula_cov;
    let solvable = coverage >= 0.35 && note_hits >= 2;
    emit_research_event(
        db,
        ctx,
        "solvability_checked",
        2,
        "gate",
        Some(&format!("notes={note_hits}, formulas={formula_hits}")),
        Some(&format!("coverage={coverage:.2}, solvable={solvable}")),
        None,
        None,
    )
    .await;

    if !solvable {
        let missing = missing_knowledge_hints(&tokens, note_hits, formula_hits);
        let uns = ResearchResult {
            title: format!("Unsolvable with current library: {problem}"),
            summary: "Unable to solve with available vault knowledge.".to_string(),
            markdown_body: format!(
                "## Status\nUNSOLVABLE_INSUFFICIENT_KNOWLEDGE\n\n## Why\n- Query grounding in local notes is below threshold.\n- Required formulas/prerequisites are missing.\n\n## Missing Knowledge\n{}\n\n## Recommendation\nAdd source notes covering the missing topics and rerun.",
                missing
                    .iter()
                    .map(|m| format!("- {m}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            references: Vec::new(),
            open_questions: missing,
            confidence: 0.0,
        };
        let artifact = vault.write_research(&uns).await?;
        emit_research_event(
            db,
            ctx,
            "finalized",
            3,
            "unsolved",
            None,
            Some("unsolvable: insufficient knowledge"),
            None,
            Some(&artifact.to_string_lossy()),
        )
        .await;
        db.finish_agent_task(task.id).await?;
        return Ok(());
    }

    let rctx = ResearchContext {
        problem: problem.clone(),
        max_iterations,
        skills_scope,
        topic_context,
        formula_context,
    };
    emit_research_event(
        db,
        ctx,
        "llm_call",
        3,
        "solve",
        Some(&problem),
        None,
        None,
        None,
    )
    .await;
    match agent.run(&rctx).await {
        Ok(out) => {
            let artifact = vault.write_research(&out).await?;
            emit_research_event(
                db,
                ctx,
                "finalized",
                4,
                "finalize",
                None,
                Some("research draft written"),
                None,
                Some(&artifact.to_string_lossy()),
            )
            .await;
            db.finish_agent_task(task.id).await?;
        }
        Err(e) => {
            emit_research_event(
                db,
                ctx,
                "failed",
                4,
                "failed",
                None,
                Some(&e.to_string()),
                None,
                None,
            )
            .await;
            db.fail_agent_task(task.id, &e.to_string()).await?;
        }
    }
    Ok(())
}

fn tokenize_problem(problem: &str) -> Vec<String> {
    let mut out = Vec::new();
    for tok in problem
        .split(|c: char| !c.is_alphanumeric())
        .map(str::trim)
        .filter(|t| t.len() >= 4)
    {
        let t = tok.to_lowercase();
        if !out.contains(&t) {
            out.push(t);
        }
        if out.len() >= 24 {
            break;
        }
    }
    out
}

async fn gather_problem_context(
    vault: &Arc<VaultWriter>,
    tokens: &[String],
) -> (Vec<String>, usize) {
    let mut contexts = Vec::new();
    let mut hits = 0usize;
    let mut stack = vec![vault.vault_dir().to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(mut entries) = tokio::fs::read_dir(&dir).await else {
            continue;
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            let Ok(ft) = entry.file_type().await else {
                continue;
            };
            if ft.is_dir() {
                if path.file_name().and_then(|n| n.to_str()) == Some("Generated") {
                    continue;
                }
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let Ok(content) = tokio::fs::read_to_string(&path).await else {
                continue;
            };
            let low = content.to_lowercase();
            let matched = tokens.iter().filter(|t| low.contains(t.as_str())).count();
            if matched == 0 {
                continue;
            }
            hits += 1;
            let rel = path
                .strip_prefix(vault.vault_dir())
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.to_string_lossy().to_string());
            let snippet = content_body(&content).chars().take(900).collect::<String>();
            contexts.push(format!("### {rel}\n{snippet}"));
            if contexts.len() >= 10 {
                return (contexts, hits);
            }
        }
    }
    (contexts, hits)
}

async fn gather_formula_context(db: &Db, tokens: &[String], limit: usize) -> Vec<String> {
    let Ok(rows) = db.list_formulas().await else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for row in rows {
        let mut hay = row.latex_norm.to_lowercase();
        if let Some(c) = &row.context {
            hay.push(' ');
            hay.push_str(&c.to_lowercase());
        }
        if !tokens.iter().any(|t| hay.contains(t.as_str())) {
            continue;
        }
        out.push(format!(
            "$${}$$ [{}] {}",
            row.latex,
            row.note_rel_path,
            row.context.unwrap_or_default()
        ));
        if out.len() >= limit {
            break;
        }
    }
    out
}

fn missing_knowledge_hints(
    tokens: &[String],
    note_hits: usize,
    formula_hits: usize,
) -> Vec<String> {
    let mut missing = Vec::new();
    if note_hits < 2 {
        missing.push(
            "Add foundational notes for the target domain and benchmark variants.".to_string(),
        );
    }
    if formula_hits == 0 {
        missing.push(
            "Add core mathematical formulations/objective functions relevant to the problem."
                .to_string(),
        );
    }
    if !tokens.is_empty() {
        missing.push(format!(
            "Create notes explicitly covering: {}.",
            tokens
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    missing
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
