//! Literature-search agent — thin Tavily client used only by the
//! BridgeFinder's iter-2 refinement.
//!
//! Budget discipline is paramount: the Tavily free tier caps at
//! **1000 req/month**, which is the scarcest resource in the whole
//! system (Gemma gives us 45K/month on the same budget). Every call
//! debits `search_usage` for the current calendar month; once the
//! ceiling is hit, [`LiteratureSearchAgent::search`] returns an empty
//! [`SearchResult`] with `used_budget = false` and the caller degrades
//! gracefully (Bridge proceeds to its critique iter without external
//! citations).
//!
//! Only the six allow-listed academic domains from
//! `cfg.tavily_domains` are queried. No general-web search ever goes
//! through this path.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::{
    config::Config,
    db::Db,
    error::{AppError, AppResult},
};

const TAVILY_ENDPOINT: &str = "https://api.tavily.com/search";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub source_domain: String,
    #[serde(default)]
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub hits: Vec<SearchHit>,
    /// True iff we actually spent a Tavily call. False means budget
    /// exhausted or agent disabled — caller should degrade.
    pub used_budget: bool,
}

// --- Tavily wire types -------------------------------------------------------

#[derive(Serialize)]
struct TavilyRequest<'a> {
    api_key: &'a str,
    query: &'a str,
    search_depth: &'a str,
    include_domains: &'a [String],
    max_results: u8,
}

#[derive(Deserialize)]
struct TavilyResponse {
    #[serde(default)]
    results: Vec<TavilyResult>,
}

#[derive(Deserialize)]
struct TavilyResult {
    #[serde(default)]
    title: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    content: String,
    #[serde(default)]
    score: f32,
}

// --- Agent -------------------------------------------------------------------

#[derive(Clone)]
pub struct LiteratureSearchAgent {
    http: reqwest::Client,
    api_key: Option<String>,
    monthly_limit: u32,
    domains: Vec<String>,
    max_results: u8,
    db: Db,
}

impl LiteratureSearchAgent {
    pub fn new(cfg: &Config, db: Db) -> AppResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::other(format!("build tavily client: {e}")))?;
        Ok(Self {
            http,
            api_key: cfg.tavily_api_key.clone(),
            monthly_limit: cfg.tavily_monthly_limit,
            domains: cfg.tavily_domains.clone(),
            max_results: cfg.tavily_max_results,
            db,
        })
    }

    #[tracing::instrument(level = "info", skip(self), fields(q = %query))]
    pub async fn search(&self, query: &str) -> AppResult<SearchResult> {
        let Some(ref key) = self.api_key else {
            debug!("tavily disabled (no api key)");
            return Ok(SearchResult {
                query: query.into(),
                hits: vec![],
                used_budget: false,
            });
        };

        let used = self.db.search_usage_this_month().await? as u32;
        if used >= self.monthly_limit {
            warn!(used, cap = self.monthly_limit, "tavily monthly cap reached");
            return Ok(SearchResult {
                query: query.into(),
                hits: vec![],
                used_budget: false,
            });
        }

        let body = TavilyRequest {
            api_key: key,
            query,
            search_depth: "basic",
            include_domains: &self.domains,
            max_results: self.max_results,
        };

        let resp = self
            .http
            .post(TAVILY_ENDPOINT)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::other(format!("tavily post: {e}")))?;
        if !resp.status().is_success() {
            let s = resp.status();
            let t = resp.text().await.unwrap_or_default();
            return Err(AppError::other(format!("tavily {s}: {t}")));
        }
        let parsed: TavilyResponse = resp
            .json()
            .await
            .map_err(|e| AppError::other(format!("tavily json: {e}")))?;
        self.db.increment_search_usage().await?;

        let hits = parsed
            .results
            .into_iter()
            .map(|r| {
                let domain = domain_of(&r.url);
                SearchHit {
                    title: r.title,
                    url: r.url,
                    snippet: r.content,
                    source_domain: domain,
                    score: r.score,
                }
            })
            .collect::<Vec<_>>();
        debug!(hits = hits.len(), "tavily ok");
        Ok(SearchResult {
            query: query.into(),
            hits,
            used_budget: true,
        })
    }
}

fn domain_of(url: &str) -> String {
    url.split("://")
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .map(|s| s.trim_start_matches("www.").to_string())
        .unwrap_or_default()
}
