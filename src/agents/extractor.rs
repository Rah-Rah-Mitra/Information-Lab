//! Extractor agent: PDF chunks → strict KG JSON via Gemma 4 26B (light tier)
//! with `StructuredOutputFormat`.
//!
//! Gates every call through the shared [`Limiter`] on `Role::Extractor`
//! (light tier, independent 15 RPM bucket from the heavy-tier research
//! agents).

use std::sync::Arc;

use autoagents::llm::{
    backends::google::Google,
    builder::LLMBuilder,
    chat::{ChatMessage, ChatProvider, StructuredOutputFormat},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::debug;

use crate::{
    config::Config,
    db::Db,
    error::{AppError, AppResult},
    limiter::{Limiter, Role},
};

use super::{record_agent_call, scrub_llm_text, truncate, AgentCall, KG_EXTRACTOR_SKILL, OBSIDIAN_WRITER_SKILL};

// ----------------------------------------------------------------------------
// Output schema
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub source: String,
    pub target: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgOutput {
    /// Short Title-Case noun phrase naming the note. Drives filename + index.
    pub title: String,
    /// One-sentence (≤160 char) summary shown in INDEX.md next to the title.
    pub summary: String,
    pub tags: Vec<String>,
    pub entities: Vec<String>,
    pub relationships: Vec<Relationship>,
    pub markdown_snippet: String,

    #[serde(skip)]
    pub tokens_sent: i64,
    #[serde(skip)]
    pub tokens_received: i64,
}

/// JSON schema handed to Gemma 4 via `response_schema`.
fn kg_schema() -> Value {
    json!({
        "type": "object",
        "required": ["title", "summary", "tags", "entities", "relationships", "markdown_snippet"],
        "properties": {
            "title":            { "type": "string", "description": "3–8 word Title-Case note title" },
            "summary":          { "type": "string", "description": "2–3 sentence summary for the index (NOT a keyword list)" },
            "tags":             { "type": "array", "items": { "type": "string" } },
            "entities":         { "type": "array", "items": { "type": "string" } },
            "relationships": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["source", "target", "description"],
                    "properties": {
                        "source":      { "type": "string" },
                        "target":      { "type": "string" },
                        "description": { "type": "string" }
                    }
                }
            },
            "markdown_snippet": { "type": "string" }
        }
    })
}

// ----------------------------------------------------------------------------
// Knowledge graph extractor (schema-forced single-shot call)
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct KnowledgeGraphAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    db: Db,
    model: String,
}

impl KnowledgeGraphAgent {
    pub fn new(cfg: &Config, limiter: Arc<Limiter>, db: Db) -> AppResult<Self> {
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(cfg.light_model.clone())
            .temperature(0.2)
            .timeout_seconds(120)
            .build()
            .map_err(|e| AppError::other(format!("build Google llm: {e}")))?;

        Ok(Self {
            llm,
            limiter,
            db,
            model: cfg.light_model.clone(),
        })
    }

    #[allow(dead_code)] // exposed for future status reporting
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Single call: rate-limited through shared `Limiter`, schema-forced, parsed.
    #[tracing::instrument(
        level = "info",
        skip(self, batched_text),
        fields(
            bytes = batched_text.len(),
            model = %self.model,
            agent.role = "extractor",
            agent.tier = "light"
        )
    )]
    pub async fn extract(&self, batched_text: &str) -> AppResult<KgOutput> {
        let _permit = self.limiter.admit(Role::Extractor).await?;

        let system = format!("{KG_EXTRACTOR_SKILL}\n\n---\n\n{OBSIDIAN_WRITER_SKILL}");
        let user_content = format!("{system}\n\n---\n\n# Input documents\n\n{batched_text}");

        let messages = vec![ChatMessage::user().content(user_content.clone()).build()];

        let schema = StructuredOutputFormat {
            name: "kg_output".to_string(),
            description: Some("Knowledge graph extraction result".into()),
            schema: Some(kg_schema()),
            strict: Some(true),
        };

        let started = std::time::Instant::now();
        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("gemma chat: {e}")))?;

        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty chat response".into()))?;

        let (tokens_sent, tokens_received) = record_agent_call(
            &self.db,
            AgentCall {
                role: Role::Extractor,
                input: &user_content,
                output: &text,
                thinking: None,
                payload_json: None,
                started,
            },
        )
        .await?;

        let mut parsed: KgOutput = serde_json::from_str(&text)
            .map_err(|e| AppError::Schema(format!("parse kg json: {e} :: {}", truncate(&text, 400))))?;

        parsed.title = scrub_llm_text(&parsed.title);
        parsed.summary = scrub_llm_text(&parsed.summary);
        parsed.markdown_snippet = scrub_llm_text(&parsed.markdown_snippet);
        parsed.tokens_sent = tokens_sent;
        parsed.tokens_received = tokens_received;
        debug!(bytes = text.len(), "kg extraction ok");
        Ok(parsed)
    }
}
