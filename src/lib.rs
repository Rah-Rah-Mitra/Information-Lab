//! Library surface for edge-kg-agent. Exposes the internal modules so
//! integration tests under `tests/` can reach the public types. The
//! binary entry point lives in `src/main.rs`.

pub mod agents;
pub mod config;
pub mod db;
pub mod error;
pub mod formula_detect;
pub mod ingest;
pub mod limiter;
pub mod orchestrator;
pub mod scheduler;
pub mod status;
pub mod telemetry;
pub mod vault;
pub mod watcher;
pub mod workflow;
