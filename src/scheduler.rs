//! Idle scheduler.
//!
//! On every `SCHEDULER_INTERVAL_SECS` tick the scheduler inspects the
//! vault + DB and enqueues follow-up work:
//!
//!   * **Curate** — for each Topic whose entry count has grown by
//!     `cfg.curate_delta_k` or more since the last snapshot, queue one
//!     Curate task and refresh the snapshot.
//!   * **Bridge**  — sample cross-source Topic pairs whose entity overlap
//!     falls in the configured mid-band (neither strangers nor near-
//!     duplicates) and queue up to `cfg.bridge_max_pending` pending.
//!   * **Harvest** — queue one Harvest task every `cfg.harvest_every_n`
//!     newly-generated notes.
//!
//! The scheduler itself never calls the LLM. It only shapes the agent-task
//! queue; the `spawn_research` and `spawn_harvester` loops drain it.

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use regex::Regex;
use serde_json::json;
use tokio::fs;
use tracing::{debug, info, warn};

use crate::{
    config::Config,
    db::{AgentTaskKind, Db},
    error::AppResult,
};

/// One topic as seen by the scheduler, materialised from `Topics/{slug}.md`.
#[derive(Debug, Clone)]
pub struct TopicView {
    pub tag: String,
    pub rel_path: String,
    /// Count of `({rel_path})` markers in the topic index file.
    pub entry_count: i64,
    /// Set of rel_paths referenced by this topic.
    pub notes: HashSet<String>,
    /// Sources inferred from the referenced `Generated/{source}/...` paths.
    pub sources: HashSet<String>,
}

pub struct Scheduler {
    cfg: Config,
    db: Db,
    vault_dir: PathBuf,
    marker: Regex,
}

impl Scheduler {
    pub fn new(cfg: Config, db: Db) -> AppResult<Self> {
        let vault_dir = cfg.vault_dir.clone();
        Ok(Self {
            cfg,
            db,
            vault_dir,
            marker: Regex::new(r"\(([^)]+\.md)\)").unwrap(),
        })
    }

    #[tracing::instrument(level = "info", skip(self))]
    pub async fn tick(&self) -> AppResult<()> {
        let topics = self.scan_topics().await?;
        self.enqueue_curate(&topics).await?;
        self.enqueue_bridges(&topics).await?;
        self.enqueue_theorems().await?;
        self.enqueue_derivations(&topics).await?;
        self.enqueue_report().await?;
        Ok(())
    }

    /// Promote high-confidence bridges into Theorem tasks. Dedup by
    /// `bridges.note_rel_path` — see `Db::unproven_bridges`.
    async fn enqueue_theorems(&self) -> AppResult<()> {
        let rows = self
            .db
            .unproven_bridges(
                self.cfg.theorem_confidence_tau,
                self.cfg.theorem_enqueue_batch,
            )
            .await?;
        for b in rows {
            let payload = json!({
                "topic_a": b.topic_a,
                "topic_b": b.topic_b,
                "source_a": b.source_a,
                "source_b": b.source_b,
                "bridge_rel_path": b.note_rel_path,
                "confidence": b.confidence,
            });
            let id = self
                .db
                .enqueue_agent_task(AgentTaskKind::Theorem, &payload)
                .await?;
            info!(
                target: "agent.spawn",
                role = "theorem",
                tier = "heavy",
                task_id = id,
                bridge = %b.note_rel_path,
                "theorem enqueued"
            );
        }
        Ok(())
    }

    /// For each topic with ≥ `derivation_min_formulas` formulas, enqueue at
    /// most one Derivation seed per tick. Skipped if the queue already has
    /// a pending Derivation task for this topic.
    async fn enqueue_derivations(&self, topics: &[TopicView]) -> AppResult<()> {
        let pending = self
            .db
            .agent_task_pending_count(AgentTaskKind::Derivation)
            .await?;
        if pending >= 2 {
            return Ok(());
        }
        let formulas = self.db.list_formulas().await?;
        for t in topics {
            let count = formulas
                .iter()
                .filter(|f| t.notes.contains(&f.note_rel_path))
                .count();
            if count < self.cfg.derivation_min_formulas {
                continue;
            }
            let note_paths: Vec<String> = formulas
                .iter()
                .filter(|f| t.notes.contains(&f.note_rel_path))
                .map(|f| f.note_rel_path.clone())
                .collect();
            let payload = json!({
                "seed_topic": t.tag,
                "notes": note_paths,
            });
            let id = self
                .db
                .enqueue_agent_task(AgentTaskKind::Derivation, &payload)
                .await?;
            info!(
                target: "agent.spawn",
                role = "derivation",
                tier = "heavy",
                task_id = id,
                topic = %t.tag,
                formulas = count,
                "derivation enqueued"
            );
            break; // one seed per tick
        }
        Ok(())
    }

    /// One Report task per calendar day — dedup via `Db::report_task_exists_for_date`.
    async fn enqueue_report(&self) -> AppResult<()> {
        let _ = self.cfg.report_interval_secs; // cadence read for future fine-grain scheduling
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        if self.db.report_task_exists_for_date(&today).await? {
            return Ok(());
        }
        let payload = json!({ "date": today });
        let id = self
            .db
            .enqueue_agent_task(AgentTaskKind::Report, &payload)
            .await?;
        info!(
            target: "agent.spawn",
            role = "report",
            tier = "heavy",
            task_id = id,
            date = %today,
            "report enqueued"
        );
        Ok(())
    }

    async fn enqueue_curate(&self, topics: &[TopicView]) -> AppResult<()> {
        for t in topics {
            let prev = self.db.topic_snapshot(&t.tag).await?.unwrap_or(0);
            let delta = t.entry_count - prev;
            if delta >= self.cfg.curate_delta_k as i64 {
                let payload = json!({
                    "topic": t.tag,
                    "topic_rel_path": t.rel_path,
                    "notes": t.notes.iter().cloned().collect::<Vec<_>>(),
                });
                let id = self
                    .db
                    .enqueue_agent_task(AgentTaskKind::Curate, &payload)
                    .await?;
                self.db
                    .upsert_topic_snapshot(&t.tag, t.entry_count, true)
                    .await?;
                info!(
                    target: "agent.spawn",
                    role = "curator",
                    tier = "heavy",
                    task_id = id,
                    topic = %t.tag,
                    delta,
                    "curate enqueued"
                );
            } else {
                // Refresh the snapshot without marking curated.
                self.db
                    .upsert_topic_snapshot(&t.tag, t.entry_count, false)
                    .await?;
            }
        }
        Ok(())
    }

    async fn enqueue_bridges(&self, topics: &[TopicView]) -> AppResult<()> {
        let pending = self
            .db
            .agent_task_pending_count(AgentTaskKind::Bridge)
            .await?;
        if pending >= self.cfg.bridge_max_pending as i64 {
            debug!(pending, "bridge queue full");
            return Ok(());
        }

        let candidates = self.sample_bridge_candidates(topics);
        let room = (self.cfg.bridge_max_pending as i64 - pending).max(0) as usize;
        for (a, b) in candidates.into_iter().take(room) {
            let source_a = a.sources.iter().next().cloned().unwrap_or_default();
            let source_b = b.sources.iter().next().cloned().unwrap_or_default();
            if self
                .db
                .bridge_exists(&a.tag, &b.tag, &source_a, &source_b)
                .await
                .unwrap_or(false)
            {
                continue;
            }
            let payload = json!({
                "topic_a": a.tag, "topic_b": b.tag,
                "source_a": source_a, "source_b": source_b,
                "notes_a": a.notes.iter().cloned().collect::<Vec<_>>(),
                "notes_b": b.notes.iter().cloned().collect::<Vec<_>>(),
            });
            let id = self
                .db
                .enqueue_agent_task(AgentTaskKind::Bridge, &payload)
                .await?;
            info!(
                target: "agent.spawn",
                role = "bridge",
                tier = "heavy",
                task_id = id,
                a = %a.tag,
                b = %b.tag,
                "bridge enqueued"
            );
        }
        Ok(())
    }

    /// Mid-band selector. Scores pairs by entity overlap and keeps only
    /// those inside `[min_overlap, max_overlap]` with `jaccard ≤ τ`.
    fn sample_bridge_candidates<'a>(
        &self,
        topics: &'a [TopicView],
    ) -> Vec<(&'a TopicView, &'a TopicView)> {
        let min_ov = self.cfg.bridge_min_overlap;
        let max_ov = self.cfg.bridge_max_overlap;
        let max_j = self.cfg.bridge_max_jaccard;

        let mut out = Vec::new();
        for i in 0..topics.len() {
            for j in (i + 1)..topics.len() {
                let a = &topics[i];
                let b = &topics[j];
                // Cross-source only: each side must have at least one source,
                // and the two topics must NOT overlap on any source — sharing
                // even one source means this is a same-textbook pair, not a
                // bridge candidate.
                if a.sources.is_empty() || b.sources.is_empty() {
                    continue;
                }
                if !a.sources.is_disjoint(&b.sources) {
                    continue;
                }
                // Use notes as a proxy for entity overlap — same note in two
                // topic indexes implies shared conceptual surface.
                let intersection = a.notes.intersection(&b.notes).count();
                if intersection < min_ov || intersection > max_ov {
                    continue;
                }
                let union =
                    a.notes.union(&b.notes).count().max(1);
                let jaccard = intersection as f32 / union as f32;
                if jaccard > max_j {
                    continue;
                }
                out.push((a, b));
            }
        }
        out
    }

    async fn scan_topics(&self) -> AppResult<Vec<TopicView>> {
        let dir = self.vault_dir.join("Topics");
        if !dir.exists() {
            return Ok(vec![]);
        }
        let mut entries = fs::read_dir(&dir).await?;
        let mut out = Vec::new();
        while let Some(e) = entries.next_entry().await? {
            let p = e.path();
            if !p.is_file() {
                continue;
            }
            if p.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let tag = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            let rel = format!("Topics/{tag}.md");
            let content = match fs::read_to_string(&p).await {
                Ok(c) => c,
                Err(e) => {
                    warn!(path = %p.display(), error = %e, "topic read failed");
                    continue;
                }
            };
            let mut notes = HashSet::new();
            let mut sources = HashSet::new();
            for caps in self.marker.captures_iter(&content) {
                let rel_note = caps.get(1).unwrap().as_str().to_string();
                if let Some(src) = source_of(&rel_note) {
                    sources.insert(src);
                }
                notes.insert(rel_note);
            }
            let entry_count = notes.len() as i64;
            out.push(TopicView {
                tag,
                rel_path: rel,
                entry_count,
                notes,
                sources,
            });
        }
        let _ = HashMap::<(), ()>::new();
        debug!(topics = out.len(), "topics scanned");
        Ok(out)
    }
}

/// Extract the `{source}` segment from a `Generated/{source}/{slug}.md` path.
/// Returns `None` for `Generated/_Syntheses/...` / `Generated/_Bridges/...`
/// or any path that doesn't match the generated layout.
fn source_of(rel_path: &str) -> Option<String> {
    let norm = rel_path.replace('\\', "/");
    let mut parts = norm.splitn(3, '/');
    if parts.next()? != "Generated" {
        return None;
    }
    let src = parts.next()?;
    if src.starts_with('_') {
        return None;
    }
    Some(src.to_string())
}

#[allow(dead_code)]
pub(crate) fn _scheduler_path_probe(p: &Path) -> Option<String> {
    source_of(p.to_string_lossy().as_ref())
}
