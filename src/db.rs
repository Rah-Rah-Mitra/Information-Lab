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
    ) -> AppResult<bool> {
        let rows = sqlx::query(
            "INSERT OR IGNORE INTO documents (hash, path, byte_size) VALUES (?, ?, ?)",
        )
        .bind(hash)
        .bind(path)
        .bind(byte_size)
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows > 0)
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

    /// Reset chunks that were claimed but never finished (crash recovery).
    pub async fn requeue_orphans(&self) -> AppResult<u64> {
        let res = sqlx::query(
            "UPDATE chunks SET state = 'pending', batch_id = NULL
             WHERE state = 'batched'",
        )
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
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
