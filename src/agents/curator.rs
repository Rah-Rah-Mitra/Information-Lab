//! Topic curator: given a Topic that has gained ≥K new entries since its
//! last snapshot, read the linked notes and write a cross-textbook
//! synthesis note with extracted formulas and citations.
//!
//! The skill text (`skills/topic_curator.md`) enforces:
//!   * every formula carries a verbatim LaTeX string + at least one
//!     citation pointing at a real note path,
//!   * no invented formulas — if a formula is derived rather than quoted
//!     verbatim, the `derived: true` flag on [`Formula`] must be set.
//!
//! One [`Limiter::admit(Role::Curator)`] gate per synthesis call.

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

use super::truncate;

pub const TOPIC_CURATOR_SKILL: &str = include_str!("../../skills/topic_curator.md");

// ----------------------------------------------------------------------------
// Output schema
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formula {
    pub latex: String,
    pub symbols: Vec<String>,
    pub context_caption: String,
    pub note_rel_path: String,
    #[serde(default)]
    pub derived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub source: String,
    pub note_rel_path: String,
    #[serde(default)]
    pub anchor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicSynthesis {
    pub topic: String,
    pub sources: Vec<String>,
    pub summary: String,
    pub formulas: Vec<Formula>,
    pub key_concepts: Vec<String>,
    pub markdown_body: String,
    pub citations: Vec<Citation>,
}

/// Reference snippet handed to the curator for one note.
#[derive(Debug, Clone, Serialize)]
pub struct NoteRef {
    pub note_title: String,
    pub source: String,
    pub summary: String,
    pub markdown_snippet: String,
    pub note_rel_path: String,
}

fn topic_synthesis_schema() -> Value {
    json!({
        "type": "object",
        "required": ["topic", "sources", "summary", "formulas",
                     "key_concepts", "markdown_body", "citations"],
        "properties": {
            "topic":       { "type": "string" },
            "sources":     { "type": "array", "items": { "type": "string" } },
            "summary":     { "type": "string" },
            "formulas": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["latex", "symbols", "context_caption", "note_rel_path"],
                    "properties": {
                        "latex":           { "type": "string" },
                        "symbols":         { "type": "array", "items": { "type": "string" } },
                        "context_caption": { "type": "string" },
                        "note_rel_path":   { "type": "string" },
                        "derived":         { "type": "boolean" }
                    }
                }
            },
            "key_concepts": { "type": "array", "items": { "type": "string" } },
            "markdown_body": { "type": "string" },
            "citations": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["source", "note_rel_path"],
                    "properties": {
                        "source":        { "type": "string" },
                        "note_rel_path": { "type": "string" },
                        "anchor":        { "type": "string" }
                    }
                }
            }
        }
    })
}

// ----------------------------------------------------------------------------
// Agent
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct TopicCuratorAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    db: Db,
    #[allow(dead_code)]
    model: String,
}

impl TopicCuratorAgent {
    pub fn new(cfg: &Config, limiter: Arc<Limiter>, db: Db) -> AppResult<Self> {
        let model = cfg.model_for_role(&cfg.curator_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model.clone())
            .temperature(0.2)
            .timeout_seconds(120)
            .build()
            .map_err(|e| AppError::other(format!("build curator llm: {e}")))?;
        Ok(Self { llm, limiter, db, model })
    }

    #[tracing::instrument(
        level = "info",
        skip(self, notes),
        fields(agent.role = "curator", agent.tier = "heavy", topic = %topic, notes = notes.len())
    )]
    pub async fn curate(&self, topic: &str, notes: &[NoteRef]) -> AppResult<TopicSynthesis> {
        let _permit = self.limiter.admit(Role::Curator).await?;

        let notes_block = notes
            .iter()
            .map(|n| {
                format!(
                    "## {}\n*source:* {}\n*path:* {}\n\n{}\n",
                    n.note_title, n.source, n.note_rel_path, n.markdown_snippet
                )
            })
            .collect::<Vec<_>>()
            .join("\n---\n\n");

        let prompt = format!(
            "{TOPIC_CURATOR_SKILL}\n\n---\n\n# Topic\n{topic}\n\n# Linked notes\n\n{notes_block}"
        );

        let messages = vec![ChatMessage::user().content(prompt.clone()).build()];
        let schema = StructuredOutputFormat {
            name: "topic_synthesis".into(),
            description: Some("Cross-textbook topic synthesis".into()),
            schema: Some(topic_synthesis_schema()),
            strict: Some(true),
        };

        let started = std::time::Instant::now();
        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("curator chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty curator response".into()))?;
        let _ = super::record_agent_call(
            &self.db,
            super::AgentCall {
                role: Role::Curator,
                input: &prompt,
                output: &text,
                thinking: None,
                payload_json: None,
                started,
            },
        )
        .await;
        let parsed: TopicSynthesis = serde_json::from_str(&text)
            .map_err(|e| AppError::Schema(format!("parse synthesis: {e} :: {}", truncate(&text, 400))))?;
        debug!(formulas = parsed.formulas.len(), "curate ok");
        Ok(parsed)
    }
}
