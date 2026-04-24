//! Runtime configuration. All tunables are env-driven so the binary can ship
//! to a Pi without recompile. Defaults target the Google AI Studio free tier
//! but model names are intentionally parameterized — point this at whichever
//! endpoint the caller has access to.

use std::{path::PathBuf, time::Duration};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct Config {
    /// Syncthing-synced folder to watch for PDFs.
    pub watch_dir: PathBuf,
    /// Obsidian vault root where generated `.md` is written.
    pub vault_dir: PathBuf,
    /// SQLite database path.
    pub db_path: PathBuf,
    /// Directory for rotating log files.
    pub log_dir: PathBuf,

    /// Google AI Studio API key.
    pub api_key: String,
    /// Base URL for the generative endpoint. Reserved for custom-endpoint routing.
    #[allow(dead_code)]
    pub api_base: String,
    /// Light-tier model (Gemma 4 26B). Used by Extractor, Harvester, and
    /// ErrorRetrier. Has its own independent 15 RPM / 1.5K RPD bucket on
    /// the free tier.
    pub light_model: String,
    /// Heavy-tier model (Gemma 4 31B). Used by Curator, Bridge, and the
    /// derivation/theorem/report research agents. Own 15 RPM / 1.5K RPD
    /// bucket — running both tiers doubles effective throughput.
    pub heavy_model: String,
    /// Backwards-compatible alias (== `heavy_model`). Kept so existing
    /// code paths that referenced `reasoner_model` keep working.
    #[allow(dead_code)]
    pub reasoner_model: String,
    /// Model name for the vision router (images / diagrams).
    pub vision_model: String,

    /// Requests-per-minute ceiling for the Token Bucket (14 RPM == 1 buffer).
    pub rpm_limit: u32,
    /// Max one-liner entries in a single index file before it auto-splits
    /// into sub-indexes. Smaller caps keep agent context windows small.
    pub index_entry_cap: usize,
    /// Number of leading pages to rasterize and send to the TOC extractor.
    /// Reserved for the vision pass (TocExtractor not yet wired).
    #[allow(dead_code)]
    pub toc_page_budget: usize,
    /// Debounce for filesystem events (Syncthing partial writes).
    pub fs_debounce: Duration,
    /// Approximate token budget to accumulate before firing a batch.
    pub batch_token_target: usize,
    /// Cadence for SYSTEM_STATUS.md regeneration.
    pub status_interval: Duration,

    /// OTLP endpoint (e.g. `http://127.0.0.1:4317`). When empty, OpenTelemetry
    /// export is disabled and tracing falls back to console + file only.
    pub otlp_endpoint: Option<String>,
    /// `service.name` resource attribute for OTel.
    pub otel_service_name: String,
    /// Additional resource attributes, comma-separated `k=v` pairs.
    pub otel_resource_attributes: Option<String>,

    // -----------------------------------------------------------------------
    // Multi-agent research layer.
    // -----------------------------------------------------------------------
    /// Daily request ceiling across all roles (Gemma 4 31B free tier = 1500).
    pub rpd_limit: u32,
    /// Daily role share, integer percent; these four should sum to 100.
    pub role_share_extractor: u32,
    pub role_share_curator: u32,
    pub role_share_bridge: u32,
    pub role_share_harvester: u32,
    pub role_share_theorem: u32,
    pub role_share_derivation: u32,
    pub role_share_report: u32,
    pub role_share_formula: u32,

    /// Minimum math-density score a chunk must reach before a
    /// `FormulaExtract` task is enqueued against it. Values in `[0.0, 1.0]`;
    /// see [`crate::formula_detect::math_density_score`].
    pub formula_detect_tau: f32,
    /// Per-role model override for the formula extractor (light tier).
    pub formula_model: String,

    /// Minimum new entries in a Topic index before a curate task is enqueued.
    pub curate_delta_k: usize,
    /// Maximum Bridge tasks sitting in the queue before the scheduler stops
    /// enqueuing more candidate pairs.
    pub bridge_max_pending: usize,
    /// Hard cap on the propose → search → critique loop.
    pub bridge_max_iters: u8,
    /// Accept a bridge as soon as its confidence crosses this threshold.
    pub bridge_confidence_tau: f32,
    /// Mid-band overlap filter for Bridge candidate selection (entity count).
    pub bridge_min_overlap: usize,
    pub bridge_max_overlap: usize,
    /// Upper bound on entity Jaccard similarity. Above this, two topics are
    /// treated as near-duplicates (no bridge is interesting).
    pub bridge_max_jaccard: f32,

    /// Enqueue a Harvest task every N newly-written notes.
    #[allow(dead_code)]
    pub harvest_every_n: usize,
    /// Idle scheduler tick.
    pub scheduler_interval: Duration,
    /// Research tick (curator + bridge in parallel).
    pub research_interval: Duration,

    /// HTTP bind address for research timeline query API.
    pub research_api_bind: String,

    /// Policy for storing optional model reasoning text (`thinking`).
    ///
    /// * `retain`   — store as-is (up to `thinking_max_bytes`)
    /// * `redact`   — store `[redacted]` marker when present
    /// * `discard`  — never store
    pub thinking_redaction_policy: String,
    /// Maximum bytes persisted for optional `thinking` payloads.
    pub thinking_max_bytes: usize,

    /// Overrides for per-role models. Blank falls back to `heavy_model`.
    pub curator_model: String,
    pub bridge_model: String,
    pub theorem_model: String,
    pub derivation_model: String,
    pub report_model: String,

    // ---- ErrorRetrier (reaps chunks stuck in `state='error'`) ----
    /// Periodic cadence for the retry sweep.
    pub error_retry_interval: Duration,
    /// Hard cap on retries before a chunk is promoted to `'failed_final'`.
    pub error_retry_max: i64,
    /// Minimum age of `last_retry_at` before a chunk is eligible again.
    pub error_retry_backoff_secs: i64,
    /// Max chunks re-queued per sweep.
    pub error_retry_batch: i64,

    /// Minimum bridge confidence before a Theorem task may be enqueued.
    pub theorem_confidence_tau: f32,
    /// Max Theorem tasks enqueued per scheduler tick.
    pub theorem_enqueue_batch: i64,
    /// Daily cadence for Report tasks (seconds between enqueue checks).
    pub report_interval_secs: i64,
    /// Minimum formulas in a topic before a Derivation seed is enqueued.
    pub derivation_min_formulas: usize,

    // ---- Tavily literature search (used only by Bridge iter 2) ----
    /// API key; blank disables the search agent entirely.
    pub tavily_api_key: Option<String>,
    /// Vendor monthly ceiling.
    pub tavily_monthly_limit: u32,
    /// Soft per-day cap = monthly / 30 rounded.
    #[allow(dead_code)]
    pub tavily_daily_soft_cap: u32,
    /// Calls allowed per Bridge run.
    #[allow(dead_code)]
    pub tavily_per_bridge_cap: u32,
    /// Allow-listed academic domains (comma-separated in env).
    pub tavily_domains: Vec<String>,
    /// Results requested per Tavily call.
    pub tavily_max_results: u8,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        // Best-effort .env load. Missing file is fine.
        let _ = dotenvy::dotenv();

        Ok(Self {
            watch_dir: env_path_or("WATCH_DIR", "./public"),
            vault_dir: env_path_or("VAULT_DIR", "./public"),
            db_path: env_path_or("DB_PATH", "./.data/state.db"),
            log_dir: env_path_or("LOG_DIR", "./logs"),

            api_key: env_required("GOOGLE_API_KEY")?,
            api_base: env_or(
                "API_BASE",
                "https://generativelanguage.googleapis.com/v1beta",
            ),
            // Gemma 4 26B A4B — light tier. Own 15 RPM / 1.5K RPD budget on the
            // Google AI Studio free tier; doubles effective throughput when
            // paired with the 31B heavy tier.
            light_model: env_or("LIGHT_MODEL", "gemma-4-26b-a4b-it"),
            // Gemma 4 31B Dense — heavy reasoner (15 RPM / 1.5K RPD).
            heavy_model: env_or(
                "HEAVY_MODEL",
                // `REASONER_MODEL` is the deprecated name; still honored so
                // existing .env files keep working.
                &env_or("REASONER_MODEL", "gemma-4-31b-it"),
            ),
            reasoner_model: env_or("REASONER_MODEL", "gemma-4-31b-it"),
            // Gemini 3.1 Flash-Lite Preview — vision / image routing.
            vision_model: env_or("VISION_MODEL", "gemini-3.1-flash-lite-preview"),

            rpm_limit: env_parse("RPM_LIMIT", 14)?,
            index_entry_cap: env_parse("INDEX_ENTRY_CAP", 20_usize)?,
            toc_page_budget: env_parse("TOC_PAGE_BUDGET", 15_usize)?,
            fs_debounce: Duration::from_millis(env_parse("FS_DEBOUNCE_MS", 2000_u64)?),
            batch_token_target: env_parse("BATCH_TOKEN_TARGET", 25_000_usize)?,
            status_interval: Duration::from_secs(env_parse("STATUS_INTERVAL_SECS", 300_u64)?),

            otlp_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .ok()
                .filter(|s| !s.trim().is_empty()),
            otel_service_name: env_or("OTEL_SERVICE_NAME", "edge-kg-agent"),
            otel_resource_attributes: std::env::var("OTEL_RESOURCE_ATTRIBUTES")
                .ok()
                .filter(|s| !s.trim().is_empty()),

            rpd_limit: env_parse("RPD_LIMIT", 1500_u32)?,
            role_share_extractor: env_parse("ROLE_SHARE_EXTRACTOR", 50_u32)?,
            role_share_curator: env_parse("ROLE_SHARE_CURATOR", 15_u32)?,
            role_share_bridge: env_parse("ROLE_SHARE_BRIDGE", 12_u32)?,
            role_share_harvester: env_parse("ROLE_SHARE_HARVESTER", 5_u32)?,
            role_share_theorem: env_parse("ROLE_SHARE_THEOREM", 8_u32)?,
            role_share_derivation: env_parse("ROLE_SHARE_DERIVATION", 7_u32)?,
            role_share_report: env_parse("ROLE_SHARE_REPORT", 3_u32)?,
            role_share_formula: env_parse("ROLE_SHARE_FORMULA", 10_u32)?,
            formula_detect_tau: env_parse("FORMULA_DETECT_TAU", 0.12_f32)?,
            formula_model: env_or("FORMULA_MODEL", ""),

            curate_delta_k: env_parse("CURATE_DELTA_K", 5_usize)?,
            bridge_max_pending: env_parse("BRIDGE_MAX_PENDING", 6_usize)?,
            bridge_max_iters: env_parse("BRIDGE_MAX_ITERS", 3_u8)?,
            bridge_confidence_tau: env_parse("BRIDGE_CONFIDENCE_TAU", 0.72_f32)?,
            bridge_min_overlap: env_parse("BRIDGE_MIN_OVERLAP", 1_usize)?,
            bridge_max_overlap: env_parse("BRIDGE_MAX_OVERLAP", 5_usize)?,
            bridge_max_jaccard: env_parse("BRIDGE_MAX_JACCARD", 0.6_f32)?,

            harvest_every_n: env_parse("HARVEST_EVERY_N", 25_usize)?,
            scheduler_interval: Duration::from_secs(env_parse("SCHEDULER_INTERVAL_SECS", 60_u64)?),
            research_interval: Duration::from_secs(env_parse("RESEARCH_INTERVAL_SECS", 30_u64)?),
            research_api_bind: env_or("RESEARCH_API_BIND", "127.0.0.1:8090"),
            thinking_redaction_policy: env_or("THINKING_REDACTION_POLICY", "redact"),
            thinking_max_bytes: env_parse("THINKING_MAX_BYTES", 2048_usize)?,

            curator_model: env_or("CURATOR_MODEL", ""),
            bridge_model: env_or("BRIDGE_MODEL", ""),
            theorem_model: env_or("THEOREM_MODEL", ""),
            derivation_model: env_or("DERIVATION_MODEL", ""),
            report_model: env_or("REPORT_MODEL", ""),

            error_retry_interval: Duration::from_secs(env_parse(
                "ERROR_RETRY_INTERVAL_SECS",
                300_u64,
            )?),
            error_retry_max: env_parse("ERROR_RETRY_MAX", 3_i64)?,
            error_retry_backoff_secs: env_parse("ERROR_RETRY_BACKOFF_SECS", 300_i64)?,
            error_retry_batch: env_parse("ERROR_RETRY_BATCH", 20_i64)?,

            theorem_confidence_tau: env_parse("THEOREM_CONFIDENCE_TAU", 0.85_f32)?,
            theorem_enqueue_batch: env_parse("THEOREM_ENQUEUE_BATCH", 2_i64)?,
            report_interval_secs: env_parse("REPORT_INTERVAL_SECS", 86400_i64)?,
            derivation_min_formulas: env_parse("DERIVATION_MIN_FORMULAS", 3_usize)?,

            tavily_api_key: std::env::var("TAVILY_API_KEY")
                .ok()
                .filter(|s| !s.trim().is_empty()),
            tavily_monthly_limit: env_parse("TAVILY_MONTHLY_LIMIT", 1000_u32)?,
            tavily_daily_soft_cap: env_parse("TAVILY_DAILY_SOFT_CAP", 33_u32)?,
            tavily_per_bridge_cap: env_parse("TAVILY_PER_BRIDGE_CAP", 1_u32)?,
            tavily_domains: env_or(
                "TAVILY_DOMAINS",
                "arxiv.org,semanticscholar.org,dl.acm.org,\
                 link.springer.com,nature.com,sciencedirect.com",
            )
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
            tavily_max_results: env_parse("TAVILY_MAX_RESULTS", 5_u8)?,
        })
    }

    /// Effective model for a role — empty override falls back to the
    /// tier default.
    pub fn model_for_role(&self, override_: &str) -> String {
        self.model_for_override(override_, &self.heavy_model)
    }

    /// Resolve a role's model name: explicit override wins, otherwise use
    /// the tier default passed by the caller.
    pub fn model_for_override(&self, override_: &str, tier_default: &str) -> String {
        if override_.trim().is_empty() {
            tier_default.to_string()
        } else {
            override_.to_string()
        }
    }
}

fn env_required(key: &str) -> AppResult<String> {
    std::env::var(key).map_err(|_| AppError::MissingEnv(key.to_string()))
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_path_or(key: &str, default: &str) -> PathBuf {
    PathBuf::from(env_or(key, default))
}

fn env_parse<T>(key: &str, default: T) -> AppResult<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match std::env::var(key) {
        Ok(v) => v
            .parse::<T>()
            .map_err(|e| AppError::BadEnv(key.to_string(), e.to_string())),
        Err(_) => Ok(default),
    }
}
