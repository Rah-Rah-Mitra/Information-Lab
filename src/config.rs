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
    /// Base URL for the generative endpoint.
    pub api_base: String,
    /// Model name for the heavy reasoner (batched text).
    pub reasoner_model: String,
    /// Model name for the vision router (images / diagrams).
    pub vision_model: String,

    /// Requests-per-minute ceiling for the Token Bucket (14 RPM == 1 buffer).
    pub rpm_limit: u32,
    /// Max one-liner entries in a single index file before it auto-splits
    /// into sub-indexes. Smaller caps keep agent context windows small.
    pub index_entry_cap: usize,
    /// Number of leading pages to rasterize and send to the TOC extractor.
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
            // Gemma 4 31B Dense — heavy reasoner (unlimited TPM, 15 RPM).
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
        })
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
