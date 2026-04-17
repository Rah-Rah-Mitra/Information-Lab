//! Tools exposed to the `ResearchAgent`. Each is a small, deterministic
//! capability the LLM can call through the ReAct loop.

use std::{path::PathBuf, sync::Arc, time::Duration};

use autoagents::core::tool::{ToolCallError, ToolRuntime};
use autoagents::prelude::{ToolInputT, ToolT};
use autoagents_derive::{tool, ToolInput};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::fs;
use tracing::debug;

// ----------------------------------------------------------------------------
// Web search (Tavily). Free tier gives ~1000 search/mo which is plenty for a
// home edge pipeline. Degrades gracefully if no key is set.
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, ToolInput, Debug)]
pub struct WebSearchArgs {
    #[input(description = "The search query")]
    pub query: String,
    #[input(description = "Maximum number of results to return (1-10)")]
    pub max_results: Option<u32>,
}

#[tool(
    name = "web_search",
    description = "Search the public web for up-to-date information on a topic. \
                   Returns a list of result objects with title, url, and snippet.",
    input = WebSearchArgs
)]
#[derive(Clone)]
pub struct WebSearchTool {
    pub http: Client,
    pub api_key: Option<String>,
}

#[async_trait::async_trait]
impl ToolRuntime for WebSearchTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let typed: WebSearchArgs = serde_json::from_value(args)
            .map_err(|e| ToolCallError::RuntimeError(Box::new(e)))?;
        let max = typed.max_results.unwrap_or(5).clamp(1, 10);

        let Some(key) = &self.api_key else {
            return Ok(json!({
                "error": "web_search is disabled (set TAVILY_API_KEY to enable)",
                "results": []
            }));
        };

        let body = json!({
            "api_key": key,
            "query": typed.query,
            "search_depth": "basic",
            "max_results": max,
        });

        let resp = self
            .http
            .post("https://api.tavily.com/search")
            .json(&body)
            .send()
            .await
            .map_err(|e| ToolCallError::RuntimeError(Box::new(e)))?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(ToolCallError::RuntimeError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("tavily {status}: {text}"),
            ))));
        }

        let v: Value = resp
            .json()
            .await
            .map_err(|e| ToolCallError::RuntimeError(Box::new(e)))?;
        debug!(query = %typed.query, "web_search ok");
        Ok(v)
    }
}

// ----------------------------------------------------------------------------
// Web fetch: GET a URL, strip to plain text, cap size.
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, ToolInput, Debug)]
pub struct WebFetchArgs {
    #[input(description = "Absolute URL to fetch (http/https)")]
    pub url: String,
}

#[tool(
    name = "web_fetch",
    description = "Download a web page and return its main text content (up to ~20 KB).",
    input = WebFetchArgs
)]
#[derive(Clone)]
pub struct WebFetchTool {
    pub http: Client,
}

#[async_trait::async_trait]
impl ToolRuntime for WebFetchTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let typed: WebFetchArgs = serde_json::from_value(args)
            .map_err(|e| ToolCallError::RuntimeError(Box::new(e)))?;

        if !typed.url.starts_with("http://") && !typed.url.starts_with("https://") {
            return Err(ToolCallError::RuntimeError(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "url must start with http:// or https://",
            ))));
        }

        let resp = self
            .http
            .get(&typed.url)
            .timeout(Duration::from_secs(20))
            .send()
            .await
            .map_err(|e| ToolCallError::RuntimeError(Box::new(e)))?;

        let status = resp.status();
        let ctype = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let body = resp
            .text()
            .await
            .map_err(|e| ToolCallError::RuntimeError(Box::new(e)))?;

        let text = if ctype.contains("html") {
            html_to_text(&body)
        } else {
            body
        };

        let truncated = truncate(&text, 20_000);
        Ok(json!({
            "url": typed.url,
            "status": status.as_u16(),
            "content_type": ctype,
            "text": truncated,
        }))
    }
}

fn html_to_text(html: &str) -> String {
    let doc = scraper::Html::parse_document(html);
    // Very simple reader: join all text nodes, collapse whitespace.
    let sel = match scraper::Selector::parse("body") {
        Ok(s) => s,
        Err(_) => return html.to_string(),
    };
    let mut out = String::new();
    for el in doc.select(&sel) {
        for t in el.text() {
            let trimmed = t.trim();
            if !trimmed.is_empty() {
                out.push_str(trimmed);
                out.push(' ');
            }
        }
    }
    out
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let mut cut = max;
        while !s.is_char_boundary(cut) && cut > 0 {
            cut -= 1;
        }
        format!("{}…[truncated]", &s[..cut])
    }
}

// ----------------------------------------------------------------------------
// Vault search: grep the Obsidian vault for an existing concept note.
// Helps the agent avoid re-introducing duplicate [[wikilinks]] under new names.
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, ToolInput, Debug)]
pub struct VaultSearchArgs {
    #[input(description = "Concept name to look up in the existing knowledge graph")]
    pub concept: String,
}

#[tool(
    name = "vault_search",
    description = "Search the Obsidian vault for existing notes mentioning a concept. \
                   Returns matching note paths and short context.",
    input = VaultSearchArgs
)]
#[derive(Clone)]
pub struct VaultSearchTool {
    pub vault_dir: Arc<PathBuf>,
}

#[async_trait::async_trait]
impl ToolRuntime for VaultSearchTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let typed: VaultSearchArgs = serde_json::from_value(args)
            .map_err(|e| ToolCallError::RuntimeError(Box::new(e)))?;

        let needle_lower = typed.concept.to_lowercase();
        let mut hits = Vec::new();
        walk_vault(&self.vault_dir, &needle_lower, &mut hits, 0)
            .await
            .map_err(|e| ToolCallError::RuntimeError(Box::new(e)))?;
        hits.truncate(20);
        Ok(json!({ "concept": typed.concept, "matches": hits }))
    }
}

// async recursion via box-pin — the vault is typically shallow.
fn walk_vault<'a>(
    dir: &'a std::path::Path,
    needle_lower: &'a str,
    hits: &'a mut Vec<Value>,
    depth: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>> {
    Box::pin(async move {
        if depth > 6 {
            return Ok(());
        }
        let mut rd = match fs::read_dir(dir).await {
            Ok(rd) => rd,
            Err(_) => return Ok(()),
        };
        while let Some(ent) = rd.next_entry().await? {
            let path = ent.path();
            if path.is_dir() {
                walk_vault(&path, needle_lower, hits, depth + 1).await?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(body) = fs::read_to_string(&path).await {
                    if body.to_lowercase().contains(needle_lower) {
                        let snippet = body
                            .lines()
                            .find(|l| l.to_lowercase().contains(needle_lower))
                            .unwrap_or("")
                            .chars()
                            .take(240)
                            .collect::<String>();
                        hits.push(json!({
                            "path": path.display().to_string(),
                            "snippet": snippet,
                        }));
                        if hits.len() >= 20 {
                            return Ok(());
                        }
                    }
                }
            }
        }
        Ok(())
    })
}
