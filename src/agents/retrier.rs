//! Error-chunk retrier.
//!
//! Chunks that hit an error during extraction used to sit in `state='error'`
//! forever — `requeue_orphans` only rescues `'batched'` on startup, and the
//! reasoner loop only ever claims `'pending'`. That left stuck errors (127 /
//! 115 / 82 in prod) invisibly draining the vault.
//!
//! This agent is a **DB-only recovery loop**: no LLM call, no Limiter admit.
//! Each tick it asks [`Db::reap_error_chunks`] to do two things in one
//! transaction:
//!   * promote chunks that have exhausted their retry budget to
//!     `'failed_final'` (terminal — the status page shows them but the
//!     reasoner never picks them up again);
//!   * re-queue a bounded batch of chunks whose exponential-style backoff
//!     has elapsed, by flipping them back to `'pending'` and bumping
//!     `retry_count`.
//!
//! The Extractor then picks them up on its next batch claim.

use crate::{config::Config, db::Db, error::AppResult};

#[derive(Clone)]
pub struct ErrorRetrierAgent {
    db: Db,
    retry_max: i64,
    backoff_secs: i64,
    batch: i64,
}

/// Summary of one sweep.
#[derive(Debug, Default, Clone, Copy)]
pub struct RetryOutcome {
    pub promoted: u64,
    pub requeued: u64,
}

impl ErrorRetrierAgent {
    pub fn new(cfg: &Config, db: Db) -> Self {
        Self {
            db,
            retry_max: cfg.error_retry_max,
            backoff_secs: cfg.error_retry_backoff_secs,
            batch: cfg.error_retry_batch,
        }
    }

    #[tracing::instrument(
        level = "info",
        skip(self),
        fields(agent.role = "error_retrier", agent.tier = "light")
    )]
    pub async fn sweep(&self) -> AppResult<RetryOutcome> {
        let (promoted, requeued) = self
            .db
            .reap_error_chunks(self.retry_max, self.backoff_secs, self.batch)
            .await?;
        Ok(RetryOutcome { promoted, requeued })
    }
}
