//! Extractor agent: PDF chunks → strict KG JSON via Gemma 4 with
//! `StructuredOutputFormat`.
//!
//! This is the original `KnowledgeGraphAgent`. It is kept self-contained
//! (owns its own `governor` limiter and its own `Arc<Google>`) so that
//! `main.rs` and `orchestrator.rs` continue to compile unchanged during
//! the multi-agent split. A follow-up commit flips this over to the
//! shared [`crate::limiter::Limiter`] and shared [`AgentCtx::llm`].

use std::{num::NonZeroU32, sync::Arc};

use autoagents::llm::{
    backends::google::Google,
    builder::LLMBuilder,
    chat::{ChatMessage, ChatProvider, StructuredOutputFormat},
};
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::debug;

use crate::{
    config::Config,
    error::{AppError, AppResult},
};

use super::{truncate, KG_EXTRACTOR_SKILL, OBSIDIAN_WRITER_SKILL};

type LocalGovernor = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

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
            "summary":          { "type": "string", "description": "Single sentence, ≤160 chars, for the index" },
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
    limiter: Arc<LocalGovernor>,
    model: String,
}

impl KnowledgeGraphAgent {
    pub fn new(cfg: &Config) -> AppResult<Self> {
        let rpm = NonZeroU32::new(cfg.rpm_limit.max(1))
            .ok_or_else(|| AppError::other("rpm must be >= 1"))?;
        let limiter = Arc::new(RateLimiter::direct(Quota::per_minute(rpm)));

        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(cfg.reasoner_model.clone())
            .temperature(0.2)
            .timeout_seconds(120)
            .build()
            .map_err(|e| AppError::other(format!("build Google llm: {e}")))?;

        Ok(Self {
            llm,
            limiter,
            model: cfg.reasoner_model.clone(),
        })
    }

    #[allow(dead_code)] // exposed for future status reporting
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Single call: rate-limited, schema-forced, parsed.
    #[tracing::instrument(level = "info", skip(self, batched_text), fields(bytes = batched_text.len(), model = %self.model))]
    pub async fn extract(&self, batched_text: &str) -> AppResult<KgOutput> {
        self.limiter.until_ready().await;

        let system = format!("{KG_EXTRACTOR_SKILL}\n\n---\n\n{OBSIDIAN_WRITER_SKILL}");

        let messages = vec![
            ChatMessage::user()
                .content(format!("{system}\n\n---\n\n# Input documents\n\n{batched_text}"))
                .build(),
        ];

        let schema = StructuredOutputFormat {
            name: "kg_output".to_string(),
            description: Some("Knowledge graph extraction result".into()),
            schema: Some(kg_schema()),
            strict: Some(true),
        };

        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("gemma chat: {e}")))?;

        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty chat response".into()))?;

        let mut parsed: KgOutput = serde_json::from_str(&text)
            .map_err(|e| AppError::Schema(format!("parse kg json: {e} :: {}", truncate(&text, 400))))?;

        parsed.tokens_sent = 0;
        parsed.tokens_received = 0;
        debug!(bytes = text.len(), "kg extraction ok");
        Ok(parsed)
    }
}
