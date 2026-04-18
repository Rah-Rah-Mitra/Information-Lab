//! Agent layer.
//!
//! Currently a single [`KnowledgeGraphAgent`] that turns batched PDF text into
//! a strict KG JSON object via Gemma 4 with `StructuredOutputFormat`.
//!
//! The deep-research / ReAct stack was removed — Tavily-driven enrichment and
//! web tools are no longer part of the pipeline. The next change set will
//! decompose this module into three single-purpose agents (TOC extractor,
//! body extractor, index curator) on top of `adk-rust`.

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

pub type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

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
// Skill prompts — compiled into the binary; see `skills/*.md`.
// ----------------------------------------------------------------------------

pub const KG_EXTRACTOR_SKILL: &str = include_str!("../skills/kg_extractor.md");
pub const OBSIDIAN_WRITER_SKILL: &str = include_str!("../skills/obsidian_writer.md");

// ----------------------------------------------------------------------------
// Knowledge graph extractor (schema-forced single-shot call)
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct KnowledgeGraphAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
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

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let mut cut = max;
        while !s.is_char_boundary(cut) && cut > 0 {
            cut -= 1;
        }
        format!("{}…", &s[..cut])
    }
}
