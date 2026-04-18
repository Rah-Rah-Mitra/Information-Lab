//! Shared LLM rate limiter.
//!
//! The Google AI Studio free tier caps Gemma 4 31B at **14 safe RPM** and
//! **1500 RPD**. One global [`governor`] limiter keeps us under the per-minute
//! ceiling; per-role [`Semaphore`]s prevent one role from starving the others;
//! a per-role daily counter shapes consumption against `RPD_LIMIT`.
//!
//! Every LLM call in the pipeline — extractor, curator, bridge-finder,
//! harvester fallback — **must** go through [`Limiter::admit`]. There is no
//! secondary limiter anywhere in the codebase.

use std::{
    collections::HashMap,
    num::NonZeroU32,
    sync::Arc,
};

use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use tokio::sync::{Mutex, Semaphore};
use tracing::{instrument, warn};

use crate::{
    config::Config,
    error::{AppError, AppResult},
};

/// Logical role of a pending LLM call. Used for semaphore fairness and
/// per-role daily counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Extractor,
    Curator,
    Bridge,
    Harvester,
}

impl Role {
    #[allow(dead_code)]
    pub fn all() -> &'static [Role] {
        &[
            Role::Extractor,
            Role::Curator,
            Role::Bridge,
            Role::Harvester,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Role::Extractor => "extractor",
            Role::Curator => "curator",
            Role::Bridge => "bridge",
            Role::Harvester => "harvester",
        }
    }
}

type Governor = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

#[derive(Default)]
struct DailyCounters {
    day: String,
    used: HashMap<Role, u32>,
}

pub struct Limiter {
    global: Arc<Governor>,
    role_sems: HashMap<Role, Arc<Semaphore>>,
    targets: HashMap<Role, u32>,
    counters: Mutex<DailyCounters>,
}

impl Limiter {
    pub fn from_config(cfg: &Config) -> AppResult<Arc<Self>> {
        let rpm = NonZeroU32::new(cfg.rpm_limit.max(1))
            .ok_or_else(|| AppError::other("RPM must be >= 1"))?;
        let global = Arc::new(RateLimiter::direct(Quota::per_minute(rpm)));

        let rpd = cfg.rpd_limit.max(1);
        let shares = [
            (Role::Extractor, cfg.role_share_extractor),
            (Role::Curator, cfg.role_share_curator),
            (Role::Bridge, cfg.role_share_bridge),
            (Role::Harvester, cfg.role_share_harvester),
        ];
        let sum: u32 = shares.iter().map(|(_, s)| *s).sum::<u32>().max(1);

        let mut targets = HashMap::new();
        let mut role_sems = HashMap::new();
        for (role, share) in &shares {
            let target = (rpd as u64 * *share as u64 / sum as u64) as u32;
            targets.insert(*role, target.max(1));
            // Small concurrency cap per role: keeps 14 RPM honest even if
            // multiple research tasks fire simultaneously.
            let permits = match role {
                Role::Extractor => 1,
                Role::Curator => 1,
                Role::Bridge => 1,
                Role::Harvester => 1,
            };
            role_sems.insert(*role, Arc::new(Semaphore::new(permits)));
        }

        Ok(Arc::new(Self {
            global,
            role_sems,
            targets,
            counters: Mutex::new(DailyCounters::default()),
        }))
    }

    /// Block until the role is allowed to make one LLM call, then return a
    /// [`Permit`] that *must* be kept alive for the duration of the call.
    /// Dropping the permit releases the role's semaphore slot but does not
    /// refund the daily counter — we debit speculatively.
    #[instrument(level = "debug", skip(self), fields(role = role.as_str()))]
    pub async fn admit(&self, role: Role) -> AppResult<Permit> {
        // Daily budget check + debit.
        {
            let mut c = self.counters.lock().await;
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
            if c.day != today {
                c.day = today;
                c.used.clear();
            }
            let used = *c.used.get(&role).unwrap_or(&0);
            let target = *self.targets.get(&role).unwrap_or(&u32::MAX);
            if used >= target {
                warn!(
                    role = role.as_str(),
                    used,
                    target,
                    "daily role budget exhausted"
                );
                return Err(AppError::other(format!(
                    "daily budget exhausted for role {}",
                    role.as_str()
                )));
            }
            c.used.insert(role, used + 1);
        }

        // Fair-share concurrency cap.
        let sem = self
            .role_sems
            .get(&role)
            .cloned()
            .ok_or_else(|| AppError::other("unknown role"))?;
        let _permit_guard = sem
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| AppError::other(format!("semaphore: {e}")))?;

        // Global RPM gate.
        self.global.until_ready().await;

        Ok(Permit {
            _permit: _permit_guard,
        })
    }
}

/// Held for the duration of one LLM call. Its `Drop` releases the role
/// semaphore permit.
pub struct Permit {
    _permit: tokio::sync::OwnedSemaphorePermit,
}
