//! Shared dual-tier LLM rate limiter.
//!
//! The Google AI Studio free tier gives each Gemma 4 model its own 15 RPM /
//! 1.5K RPD bucket. Running the **light** (Gemma 4 26B) and **heavy**
//! (Gemma 4 31B) models side-by-side therefore doubles effective throughput:
//! each tier has its own independent [`governor`] instance, so a burst of
//! Extractor calls on the light tier does not starve the Curator or Bridge
//! on the heavy tier.
//!
//! Every LLM call in the pipeline **must** go through [`Limiter::admit`] —
//! the per-tier global governor, a per-role semaphore (fair-share), and the
//! per-role daily counter all live behind this one function.

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

/// Which Gemma 4 model a role runs on. Each tier has an independent RPM
/// bucket; see module docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    /// Gemma 4 26B — lightweight, used by high-volume roles.
    Light,
    /// Gemma 4 31B — heavy reasoning, used by research roles.
    Heavy,
}

impl Tier {
    pub fn as_str(self) -> &'static str {
        match self {
            Tier::Light => "light",
            Tier::Heavy => "heavy",
        }
    }
}

/// Logical role of a pending LLM call. Used for semaphore fairness,
/// per-role daily counters, and tier routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Extractor,
    Curator,
    Bridge,
    Harvester,
    Theorem,
    Derivation,
    Report,
    FormulaExtractor,
}

impl Role {
    #[allow(dead_code)]
    pub fn all() -> &'static [Role] {
        &[
            Role::Extractor,
            Role::Curator,
            Role::Bridge,
            Role::Harvester,
            Role::Theorem,
            Role::Derivation,
            Role::Report,
            Role::FormulaExtractor,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Role::Extractor => "extractor",
            Role::Curator => "curator",
            Role::Bridge => "bridge",
            Role::Harvester => "harvester",
            Role::Theorem => "theorem",
            Role::Derivation => "derivation",
            Role::Report => "report",
            Role::FormulaExtractor => "formula_extractor",
        }
    }

    /// Which model tier this role runs on.
    ///
    /// Extractor and Harvester are high-volume and tolerate a smaller
    /// context window, so they live on the Light tier. Curator, Bridge,
    /// and the research-layer agents (Theorem, Derivation, Report) all
    /// run on Heavy.
    pub fn tier(self) -> Tier {
        match self {
            Role::Extractor | Role::Harvester | Role::FormulaExtractor => Tier::Light,
            Role::Curator
            | Role::Bridge
            | Role::Theorem
            | Role::Derivation
            | Role::Report => Tier::Heavy,
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
    /// Per-tier global RPM gate. Independent tiers → independent buckets.
    tier_governors: HashMap<Tier, Arc<Governor>>,
    role_sems: HashMap<Role, Arc<Semaphore>>,
    targets: HashMap<Role, u32>,
    counters: Mutex<DailyCounters>,
}

impl Limiter {
    pub fn from_config(cfg: &Config) -> AppResult<Arc<Self>> {
        let rpm = NonZeroU32::new(cfg.rpm_limit.max(1))
            .ok_or_else(|| AppError::other("RPM must be >= 1"))?;

        // One governor per tier — independent RPM buckets.
        let mut tier_governors: HashMap<Tier, Arc<Governor>> = HashMap::new();
        for tier in [Tier::Light, Tier::Heavy] {
            tier_governors.insert(
                tier,
                Arc::new(RateLimiter::direct(Quota::per_minute(rpm))),
            );
        }

        let rpd = cfg.rpd_limit.max(1);
        let shares = [
            (Role::Extractor, cfg.role_share_extractor),
            (Role::Curator, cfg.role_share_curator),
            (Role::Bridge, cfg.role_share_bridge),
            (Role::Harvester, cfg.role_share_harvester),
            (Role::Theorem, cfg.role_share_theorem),
            (Role::Derivation, cfg.role_share_derivation),
            (Role::Report, cfg.role_share_report),
            (Role::FormulaExtractor, cfg.role_share_formula),
        ];
        let sum: u32 = shares.iter().map(|(_, s)| *s).sum::<u32>().max(1);

        let mut targets = HashMap::new();
        let mut role_sems = HashMap::new();
        for (role, share) in &shares {
            let target = (rpd as u64 * *share as u64 / sum as u64) as u32;
            targets.insert(*role, target.max(1));
            role_sems.insert(*role, Arc::new(Semaphore::new(1)));
        }

        Ok(Arc::new(Self {
            tier_governors,
            role_sems,
            targets,
            counters: Mutex::new(DailyCounters::default()),
        }))
    }

    /// Block until the role is allowed to make one LLM call, then return a
    /// [`Permit`] that *must* be kept alive for the duration of the call.
    /// Dropping the permit releases the role's semaphore slot but does not
    /// refund the daily counter — we debit speculatively.
    #[instrument(
        level = "debug",
        skip(self),
        fields(role = role.as_str(), tier = role.tier().as_str())
    )]
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
                    tier = role.tier().as_str(),
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

        // Tier-scoped RPM gate.
        let gov = self
            .tier_governors
            .get(&role.tier())
            .ok_or_else(|| AppError::other("unknown tier"))?;
        gov.until_ready().await;

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
