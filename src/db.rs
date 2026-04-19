//! SQLite-backed state. Survives reboots; recovers pending chunks on startup.

use std::path::Path;

use chrono::{NaiveDate, Utc};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions, SqlitePool,
};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PendingChunk {
    pub id: i64,
    pub doc_hash: String,
    pub page_start: i64,
    pub page_end: i64,
    pub content: String,
    pub token_estimate: i64,
}

#[derive(Debug, Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    /// Open the pool, enable WAL, run migrations.
    pub async fn open(path: &Path) -> AppResult<Self> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                tokio::fs::create_dir_all(parent).await?;
            }
        }

        let opts = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(std::time::Duration::from_secs(5))
            .log_statements(tracing::log::LevelFilter::Debug);

        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .connect_with(opts)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    /// Record a newly-seen document. Returns `true` if it was new.
    pub async fn insert_document(
        &self,
        hash: &str,
        path: &str,
        byte_size: i64,
        source_name: &str,
    ) -> AppResult<bool> {
        let rows = sqlx::query(
            "INSERT OR IGNORE INTO documents (hash, path, byte_size, source_name)
             VALUES (?, ?, ?, ?)",
        )
        .bind(hash)
        .bind(path)
        .bind(byte_size)
        .bind(source_name)
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows > 0)
    }

    /// Look up the human-readable source name for a document by hash.
    pub async fn document_source_name(&self, hash: &str) -> AppResult<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT source_name FROM documents WHERE hash = ?")
                .bind(hash)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    /// If all chunks for this document are in a terminal state (done/error),
    /// stamp `completed_at`. Returns true if this call was the one that
    /// flipped it.
    pub async fn maybe_mark_document_complete(&self, hash: &str) -> AppResult<bool> {
        let remaining: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM chunks
             WHERE doc_hash = ? AND state IN ('pending', 'batched')",
        )
        .bind(hash)
        .fetch_one(&self.pool)
        .await?;
        if remaining > 0 {
            return Ok(false);
        }
        let res = sqlx::query(
            "UPDATE documents SET completed_at = datetime('now')
             WHERE hash = ? AND completed_at IS NULL",
        )
        .bind(hash)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }

    /// Per-document progress rollup for the status page. Aggregates chunk
    /// state on read — cheap given `idx_chunks_doc` + `idx_chunks_state`.
    pub async fn list_document_progress(&self) -> AppResult<Vec<DocumentProgress>> {
        let rows: Vec<DocumentProgress> = sqlx::query_as(
            r#"
            SELECT
                d.hash                                                                 AS hash,
                d.path                                                                 AS path,
                d.source_name                                                          AS source_name,
                d.byte_size                                                            AS byte_size,
                d.discovered_at                                                        AS discovered_at,
                d.completed_at                                                         AS completed_at,
                COUNT(c.id)                                                            AS chunks_total,
                COALESCE(SUM(CASE WHEN c.state = 'done'    THEN 1 ELSE 0 END), 0)      AS chunks_done,
                COALESCE(SUM(CASE WHEN c.state = 'pending' THEN 1 ELSE 0 END), 0)      AS chunks_pending,
                COALESCE(SUM(CASE WHEN c.state = 'batched' THEN 1 ELSE 0 END), 0)      AS chunks_batched,
                COALESCE(SUM(CASE WHEN c.state = 'error'   THEN 1 ELSE 0 END), 0)      AS chunks_error
            FROM documents d
            LEFT JOIN chunks c ON c.doc_hash = d.hash
            GROUP BY d.hash
            ORDER BY d.completed_at IS NULL DESC, d.discovered_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Return the distinct doc_hashes touched by a batch (so the caller
    /// can re-check document completion after `mark_batch_done`).
    pub async fn batch_doc_hashes(&self, batch_id: &str) -> AppResult<Vec<String>> {
        let rows: Vec<String> = sqlx::query_scalar(
            "SELECT DISTINCT doc_hash FROM chunks WHERE batch_id = ?",
        )
        .bind(batch_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn mark_extracted(&self, hash: &str) -> AppResult<()> {
        sqlx::query("UPDATE documents SET extracted_at = datetime('now') WHERE hash = ?")
            .bind(hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert_chunk(
        &self,
        doc_hash: &str,
        page_start: i64,
        page_end: i64,
        content: &str,
        token_estimate: i64,
    ) -> AppResult<i64> {
        let rec = sqlx::query_scalar::<_, i64>(
            "INSERT INTO chunks (doc_hash, page_start, page_end, content, token_estimate, state)
             VALUES (?, ?, ?, ?, ?, 'pending') RETURNING id",
        )
        .bind(doc_hash)
        .bind(page_start)
        .bind(page_end)
        .bind(content)
        .bind(token_estimate)
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    /// Claim up to `max_tokens` worth of pending chunks under a new batch id.
    /// Uses a transaction so concurrent workers don't double-claim.
    pub async fn claim_batch(
        &self,
        max_tokens: i64,
        batch_id: &str,
    ) -> AppResult<Vec<PendingChunk>> {
        let mut tx = self.pool.begin().await?;

        let candidates: Vec<PendingChunk> = sqlx::query_as(
            "SELECT id, doc_hash, page_start, page_end, content, token_estimate
             FROM chunks WHERE state = 'pending' ORDER BY id LIMIT 512",
        )
        .fetch_all(&mut *tx)
        .await?;

        let mut picked = Vec::new();
        let mut total = 0_i64;
        for c in candidates {
            if total + c.token_estimate > max_tokens && !picked.is_empty() {
                break;
            }
            total += c.token_estimate;
            picked.push(c);
        }

        if picked.is_empty() {
            tx.rollback().await?;
            return Ok(Vec::new());
        }

        for c in &picked {
            sqlx::query(
                "UPDATE chunks SET state = 'batched', batch_id = ?, updated_at = datetime('now')
                 WHERE id = ?",
            )
            .bind(batch_id)
            .bind(c.id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(picked)
    }

    pub async fn mark_batch_done(&self, batch_id: &str) -> AppResult<()> {
        sqlx::query("UPDATE chunks SET state = 'done', updated_at = datetime('now') WHERE batch_id = ?")
            .bind(batch_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn mark_batch_error(&self, batch_id: &str, err: &str) -> AppResult<()> {
        sqlx::query(
            "UPDATE chunks SET state = 'error', last_error = ?, updated_at = datetime('now')
             WHERE batch_id = ?",
        )
        .bind(err)
        .bind(batch_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Reset chunks and agent tasks that were claimed but never finished
    /// (crash recovery). Called once on startup.
    pub async fn requeue_orphans(&self) -> AppResult<u64> {
        let chunks_reset = sqlx::query(
            "UPDATE chunks SET state = 'pending', batch_id = NULL
             WHERE state = 'batched'",
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        let tasks_reset = sqlx::query(
            "UPDATE agent_tasks SET state = 'pending', batch_id = NULL
             WHERE state = 'running'",
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(chunks_reset + tasks_reset)
    }

    // -----------------------------------------------------------------------
    // Agent-task queue (Curate / Bridge / Harvest — Extract stays chunk-based)
    // -----------------------------------------------------------------------

    pub async fn enqueue_agent_task(
        &self,
        kind: AgentTaskKind,
        payload: &serde_json::Value,
    ) -> AppResult<i64> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO agent_tasks (kind, payload) VALUES (?, ?) RETURNING id",
        )
        .bind(kind.as_str())
        .bind(payload.to_string())
        .fetch_one(&self.pool)
        .await?;
        Ok(id)
    }

    /// Claim a single pending task of the given kind, flipping it to running.
    /// Returns `None` when the queue is empty for that kind.
    pub async fn claim_agent_task(
        &self,
        kind: AgentTaskKind,
        batch_id: &str,
    ) -> AppResult<Option<AgentTaskRow>> {
        let mut tx = self.pool.begin().await?;
        let row: Option<AgentTaskRow> = sqlx::query_as(
            "SELECT id, kind, payload, state, batch_id, last_error,
                    created_at, completed_at
             FROM agent_tasks
             WHERE state = 'pending' AND kind = ?
             ORDER BY id LIMIT 1",
        )
        .bind(kind.as_str())
        .fetch_optional(&mut *tx)
        .await?;
        let Some(task) = row else {
            tx.rollback().await?;
            return Ok(None);
        };
        sqlx::query(
            "UPDATE agent_tasks SET state = 'running', batch_id = ? WHERE id = ?",
        )
        .bind(batch_id)
        .bind(task.id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(Some(task))
    }

    pub async fn finish_agent_task(&self, id: i64) -> AppResult<()> {
        sqlx::query(
            "UPDATE agent_tasks SET state = 'done',
                completed_at = datetime('now') WHERE id = ?",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn fail_agent_task(&self, id: i64, err: &str) -> AppResult<()> {
        sqlx::query(
            "UPDATE agent_tasks SET state = 'error', last_error = ?,
                completed_at = datetime('now') WHERE id = ?",
        )
        .bind(err)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn agent_task_pending_count(
        &self,
        kind: AgentTaskKind,
    ) -> AppResult<i64> {
        let n: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM agent_tasks
             WHERE kind = ? AND state IN ('pending','running')",
        )
        .bind(kind.as_str())
        .fetch_one(&self.pool)
        .await?;
        Ok(n)
    }

    /// Bridges at or above `min_confidence` that have no Theorem task yet.
    pub async fn unproven_bridges(
        &self,
        min_confidence: f32,
        limit: i64,
    ) -> AppResult<Vec<BridgeRow>> {
        let rows: Vec<BridgeRow> = sqlx::query_as(
            "SELECT topic_a, topic_b, source_a, source_b, confidence, note_rel_path
             FROM bridges
             WHERE confidence >= ?
               AND NOT EXISTS (
                   SELECT 1 FROM agent_tasks
                   WHERE kind = 'Theorem'
                     AND json_extract(payload, '$.bridge_rel_path') = bridges.note_rel_path
               )
             ORDER BY confidence DESC
             LIMIT ?",
        )
        .bind(min_confidence)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Has a Report task already been enqueued for this date (YYYY-MM-DD)?
    pub async fn report_task_exists_for_date(&self, date: &str) -> AppResult<bool> {
        let n: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM agent_tasks
             WHERE kind = 'Report'
               AND json_extract(payload, '$.date') = ?",
        )
        .bind(date)
        .fetch_one(&self.pool)
        .await?;
        Ok(n > 0)
    }

    // -----------------------------------------------------------------------
    // Topic snapshots — scheduler uses delta-vs-snapshot to decide curation.
    // -----------------------------------------------------------------------

    pub async fn topic_snapshot(&self, topic: &str) -> AppResult<Option<i64>> {
        Ok(sqlx::query_scalar(
            "SELECT entry_count FROM topic_snapshots WHERE topic = ?",
        )
        .bind(topic)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn upsert_topic_snapshot(
        &self,
        topic: &str,
        entry_count: i64,
        mark_curated: bool,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO topic_snapshots (topic, entry_count, last_curated_at, last_snapshot_at)
             VALUES (?, ?, CASE WHEN ? THEN datetime('now') ELSE NULL END, datetime('now'))
             ON CONFLICT(topic) DO UPDATE SET
                 entry_count = excluded.entry_count,
                 last_snapshot_at = datetime('now'),
                 last_curated_at = CASE WHEN ? THEN datetime('now') ELSE last_curated_at END",
        )
        .bind(topic)
        .bind(entry_count)
        .bind(mark_curated)
        .bind(mark_curated)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Bridges — avoid re-proposing the same cross-source pair.
    // -----------------------------------------------------------------------

    pub async fn bridge_exists(
        &self,
        topic_a: &str,
        topic_b: &str,
        source_a: &str,
        source_b: &str,
    ) -> AppResult<bool> {
        let n: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM bridges
             WHERE topic_a = ? AND topic_b = ? AND source_a = ? AND source_b = ?",
        )
        .bind(topic_a)
        .bind(topic_b)
        .bind(source_a)
        .bind(source_b)
        .fetch_one(&self.pool)
        .await?;
        Ok(n > 0)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn insert_bridge(
        &self,
        topic_a: &str,
        topic_b: &str,
        source_a: &str,
        source_b: &str,
        confidence: f32,
        iterations: i64,
        note_rel_path: &str,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO bridges
             (topic_a, topic_b, source_a, source_b, confidence, iterations, note_rel_path)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(topic_a)
        .bind(topic_b)
        .bind(source_a)
        .bind(source_b)
        .bind(confidence)
        .bind(iterations)
        .bind(note_rel_path)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Formula corpus.
    // -----------------------------------------------------------------------

    pub async fn upsert_formula(
        &self,
        latex_norm: &str,
        latex: &str,
        symbols: &str,
        context: &str,
        note_rel_path: &str,
    ) -> AppResult<bool> {
        let res = sqlx::query(
            "INSERT OR IGNORE INTO formulas
             (latex_norm, latex, symbols, context, note_rel_path)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(latex_norm)
        .bind(latex)
        .bind(symbols)
        .bind(context)
        .bind(note_rel_path)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn list_formulas(&self) -> AppResult<Vec<FormulaRow>> {
        Ok(sqlx::query_as(
            "SELECT id, latex_norm, latex, symbols, context, note_rel_path, first_seen_at
             FROM formulas ORDER BY first_seen_at DESC",
        )
        .fetch_all(&self.pool)
        .await?)
    }

    // -----------------------------------------------------------------------
    // Tavily monthly budget.
    // -----------------------------------------------------------------------

    pub async fn search_usage_this_month(&self) -> AppResult<i64> {
        let month = Utc::now().format("%Y-%m").to_string();
        Ok(sqlx::query_scalar(
            "SELECT calls FROM search_usage WHERE month = ?",
        )
        .bind(&month)
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or(0))
    }

    pub async fn increment_search_usage(&self) -> AppResult<()> {
        let month = Utc::now().format("%Y-%m").to_string();
        sqlx::query(
            "INSERT INTO search_usage (month, calls, last_call_at)
             VALUES (?, 1, datetime('now'))
             ON CONFLICT(month) DO UPDATE SET
                 calls = calls + 1, last_call_at = datetime('now')",
        )
        .bind(&month)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Reap chunks sitting in `state='error'`:
    ///
    /// * Chunks that have hit `retry_max` retries get promoted to
    ///   `'failed_final'` (terminal — no more retries, but still visible on
    ///   the status page).
    /// * Chunks whose `last_retry_at` is `NULL` or older than
    ///   `backoff_secs` are re-queued: `state='pending'`, `batch_id=NULL`,
    ///   `last_error=NULL`, `retry_count += 1`, `last_retry_at=now()`.
    ///
    /// Returns `(promoted_to_failed_final, requeued)`.
    pub async fn reap_error_chunks(
        &self,
        retry_max: i64,
        backoff_secs: i64,
        batch: i64,
    ) -> AppResult<(u64, u64)> {
        let mut tx = self.pool.begin().await?;

        // Terminal promotion: exhausted retry budget.
        let promoted = sqlx::query(
            "UPDATE chunks
             SET state = 'failed_final', updated_at = datetime('now')
             WHERE state = 'error' AND retry_count >= ?",
        )
        .bind(retry_max)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        // Requeue: pick up to `batch` chunks whose backoff has elapsed.
        // SQLite doesn't support UPDATE ... LIMIT by default; gate via
        // subquery against rowid.
        let requeued = sqlx::query(
            "UPDATE chunks
             SET state = 'pending',
                 batch_id = NULL,
                 last_error = NULL,
                 retry_count = retry_count + 1,
                 last_retry_at = datetime('now'),
                 updated_at = datetime('now')
             WHERE id IN (
                 SELECT id FROM chunks
                 WHERE state = 'error'
                   AND retry_count < ?
                   AND (last_retry_at IS NULL
                        OR last_retry_at <= datetime('now', ?))
                 ORDER BY id
                 LIMIT ?
             )",
        )
        .bind(retry_max)
        .bind(format!("-{backoff_secs} seconds"))
        .bind(batch)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        tx.commit().await?;
        Ok((promoted, requeued))
    }

    pub async fn pending_count(&self) -> AppResult<i64> {
        let n: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM chunks WHERE state = 'pending'")
                .fetch_one(&self.pool)
                .await?;
        Ok(n)
    }

    pub async fn increment_usage(
        &self,
        kind: UsageKind,
        tokens_sent: i64,
        tokens_received: i64,
    ) -> AppResult<()> {
        let today = Utc::now().date_naive().to_string();
        sqlx::query(
            "INSERT INTO api_usage (day, reasoner_calls, vision_calls, tokens_sent, tokens_received)
             VALUES (?, 0, 0, 0, 0)
             ON CONFLICT(day) DO NOTHING",
        )
        .bind(&today)
        .execute(&self.pool)
        .await?;

        let col = match kind {
            UsageKind::Reasoner => "reasoner_calls",
            UsageKind::Vision => "vision_calls",
            UsageKind::Curator => "curator_calls",
            UsageKind::Bridge => "bridge_calls",
            UsageKind::Harvester => "harvester_calls",
            UsageKind::Search => "search_calls",
        };
        // sqlx doesn't parameterize column names; we built `col` from a closed enum above.
        let sql = format!(
            "UPDATE api_usage SET {col} = {col} + 1,
                tokens_sent = tokens_sent + ?, tokens_received = tokens_received + ?
             WHERE day = ?"
        );
        sqlx::query(&sql)
            .bind(tokens_sent)
            .bind(tokens_received)
            .bind(&today)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn usage_for(&self, day: NaiveDate) -> AppResult<UsageRow> {
        let key = day.to_string();
        let row: Option<UsageRow> = sqlx::query_as(
            "SELECT day, reasoner_calls, vision_calls, tokens_sent, tokens_received
             FROM api_usage WHERE day = ?",
        )
        .bind(&key)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.unwrap_or(UsageRow {
            day: key,
            reasoner_calls: 0,
            vision_calls: 0,
            tokens_sent: 0,
            tokens_received: 0,
        }))
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // Vision variant is reserved for the stubbed vision actor.
pub enum UsageKind {
    Reasoner,
    Vision,
    Curator,
    Bridge,
    Harvester,
    Search,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)] // path/byte_size/discovered_at are populated by sqlx for future status fields.
pub struct DocumentProgress {
    pub hash: String,
    pub path: String,
    pub source_name: Option<String>,
    pub byte_size: i64,
    pub discovered_at: String,
    pub completed_at: Option<String>,
    pub chunks_total: i64,
    pub chunks_done: i64,
    pub chunks_pending: i64,
    pub chunks_batched: i64,
    pub chunks_error: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UsageRow {
    pub day: String,
    pub reasoner_calls: i64,
    pub vision_calls: i64,
    pub tokens_sent: i64,
    pub tokens_received: i64,
}

// Surface an explicit conversion target for tests/bin crates.
impl From<std::num::TryFromIntError> for AppError {
    fn from(e: std::num::TryFromIntError) -> Self {
        AppError::other(format!("int conversion: {e}"))
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // Theorem/Derivation/Report/FormulaExtract/ErrorRetry wired incrementally.
pub enum AgentTaskKind {
    Curate,
    Bridge,
    Harvest,
    Theorem,
    Derivation,
    Report,
    FormulaExtract,
    ErrorRetry,
}

impl AgentTaskKind {
    pub fn as_str(self) -> &'static str {
        match self {
            AgentTaskKind::Curate => "Curate",
            AgentTaskKind::Bridge => "Bridge",
            AgentTaskKind::Harvest => "Harvest",
            AgentTaskKind::Theorem => "Theorem",
            AgentTaskKind::Derivation => "Derivation",
            AgentTaskKind::Report => "Report",
            AgentTaskKind::FormulaExtract => "FormulaExtract",
            AgentTaskKind::ErrorRetry => "ErrorRetry",
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct AgentTaskRow {
    pub id: i64,
    pub kind: String,
    pub payload: String,
    pub state: String,
    pub batch_id: Option<String>,
    pub last_error: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BridgeRow {
    pub topic_a: String,
    pub topic_b: String,
    pub source_a: String,
    pub source_b: String,
    pub confidence: f32,
    pub note_rel_path: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct FormulaRow {
    pub id: i64,
    pub latex_norm: String,
    pub latex: String,
    pub symbols: Option<String>,
    pub context: Option<String>,
    pub note_rel_path: String,
    pub first_seen_at: String,
}
