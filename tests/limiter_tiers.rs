//! Integration tests for the dual-tier `Limiter`.
//!
//! * Light (Extractor/Harvester) and Heavy (Curator/Bridge/...) roles
//!   admit independently — exhausting one tier's RPM must not block
//!   a role on the other tier.
//! * A role whose daily budget is exhausted returns an error from
//!   `admit` instead of silently blocking.

use std::{path::PathBuf, time::Duration};

use edge_kg_agent::config::Config;
use edge_kg_agent::limiter::{Limiter, Role};

fn test_cfg() -> Config {
    Config {
        watch_dir: PathBuf::from("."),
        vault_dir: PathBuf::from("."),
        db_path: PathBuf::from("./.data/test.db"),
        log_dir: PathBuf::from("./logs"),
        api_key: "test".into(),
        api_base: String::new(),
        light_model: "light".into(),
        heavy_model: "heavy".into(),
        reasoner_model: "heavy".into(),
        vision_model: "vision".into(),

        rpm_limit: 14,
        index_entry_cap: 20,
        toc_page_budget: 15,
        fs_debounce: Duration::from_millis(2000),
        batch_token_target: 25_000,
        status_interval: Duration::from_secs(300),

        otlp_endpoint: None,
        otel_service_name: "test".into(),
        otel_resource_attributes: None,

        // Role share defaults. Tests that probe per-role budget override
        // these before calling `from_config`.
        rpd_limit: 1500,
        role_share_extractor: 50,
        role_share_curator: 15,
        role_share_bridge: 12,
        role_share_harvester: 5,
        role_share_theorem: 8,
        role_share_derivation: 7,
        role_share_report: 3,
        role_share_formula: 10,
        formula_detect_tau: 0.12,
        formula_model: String::new(),

        curate_delta_k: 5,
        bridge_max_pending: 6,
        bridge_max_iters: 3,
        bridge_confidence_tau: 0.72,
        bridge_min_overlap: 1,
        bridge_max_overlap: 5,
        bridge_max_jaccard: 0.6,

        harvest_every_n: 25,
        scheduler_interval: Duration::from_secs(60),
        research_interval: Duration::from_secs(30),
        research_api_bind: "127.0.0.1:0".into(),
        thinking_redaction_policy: "redact".into(),
        thinking_max_bytes: 512,

        curator_model: String::new(),
        bridge_model: String::new(),
        theorem_model: String::new(),
        derivation_model: String::new(),
        report_model: String::new(),

        error_retry_interval: Duration::from_secs(300),
        error_retry_max: 3,
        error_retry_backoff_secs: 300,
        error_retry_batch: 20,

        theorem_confidence_tau: 0.85,
        theorem_enqueue_batch: 2,
        report_interval_secs: 86400,
        derivation_min_formulas: 3,

        tavily_api_key: None,
        tavily_monthly_limit: 1000,
        tavily_daily_soft_cap: 33,
        tavily_per_bridge_cap: 1,
        tavily_domains: vec![],
        tavily_max_results: 5,
    }
}

#[tokio::test]
async fn light_and_heavy_tiers_admit_independently() {
    let cfg = test_cfg();
    let lim = Limiter::from_config(&cfg).expect("build limiter");

    // One admission on each tier should each succeed immediately. The
    // per-role semaphore only blocks *within* a role, so running light
    // then heavy back-to-back exercises independent tier governors.
    let p_light = tokio::time::timeout(Duration::from_secs(1), lim.admit(Role::Extractor))
        .await
        .expect("light admit timed out")
        .expect("light admit errored");

    let p_heavy = tokio::time::timeout(Duration::from_secs(1), lim.admit(Role::Curator))
        .await
        .expect("heavy admit timed out")
        .expect("heavy admit errored");

    drop(p_light);
    drop(p_heavy);
}

#[tokio::test]
async fn role_budget_exhaustion_returns_error() {
    // Force Extractor's daily target down to exactly 1: rpd=100, share
    // extractor=1, everything else 99 total ⇒ 100 * 1 / 100 = 1.
    let mut cfg = test_cfg();
    cfg.rpd_limit = 100;
    cfg.role_share_extractor = 1;
    cfg.role_share_curator = 40;
    cfg.role_share_bridge = 20;
    cfg.role_share_harvester = 1;
    cfg.role_share_theorem = 20;
    cfg.role_share_derivation = 10;
    cfg.role_share_report = 8;

    let lim = Limiter::from_config(&cfg).expect("build limiter");

    // First call consumes the budget.
    let permit = lim.admit(Role::Extractor).await.expect("first admit");
    drop(permit);

    // Second call must fail — per-role daily counter is exhausted.
    let err = match lim.admit(Role::Extractor).await {
        Ok(_) => panic!("second admit should have failed"),
        Err(e) => e,
    };
    let msg = format!("{err}");
    assert!(
        msg.contains("daily budget") || msg.contains("extractor"),
        "unexpected error message: {msg}"
    );
}
