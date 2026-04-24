//! DerivationChainAgent — heavy tier.
//!
//! Takes a set of candidate formulas (with overlapping symbols) and emits
//! a single linear derivation chain `f₁ → f₂ → … → fₙ`, each step
//! justified from the previous. Skill enforces verbatim LaTeX and
//! shared-symbol gating.

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

use super::{curator::Formula, truncate};

pub const DERIVATION_CHAIN_SKILL: &str = include_str!("../../skills/derivation_chain.md");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationStep {
    pub index: i64,
    pub latex: String,
    pub reason: String,
    pub linked_symbols: Vec<String>,
    pub source_note_rel_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationChainNote {
    pub title: String,
    pub entry_symbol: String,
    pub exit_symbol: String,
    pub steps: Vec<DerivationStep>,
    #[serde(default)]
    pub gap_reason: Option<String>,
    pub markdown_body: String,
}

fn derivation_schema() -> Value {
    json!({
        "type": "object",
        "required": ["title","entry_symbol","exit_symbol","steps","markdown_body"],
        "properties": {
            "title":        { "type": "string" },
            "entry_symbol": { "type": "string" },
            "exit_symbol":  { "type": "string" },
            "steps": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["index","latex","reason","linked_symbols","source_note_rel_path"],
                    "properties": {
                        "index":                 { "type": "integer" },
                        "latex":                 { "type": "string" },
                        "reason":                { "type": "string" },
                        "linked_symbols":        { "type": "array", "items": { "type": "string" } },
                        "source_note_rel_path":  { "type": "string" }
                    }
                }
            },
            "gap_reason":   { "type": "string" },
            "markdown_body":{ "type": "string" }
        }
    })
}

#[derive(Clone)]
pub struct DerivationChainAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    db: Db,
    #[allow(dead_code)]
    model: String,
}

impl DerivationChainAgent {
    pub fn new(cfg: &Config, limiter: Arc<Limiter>, db: Db) -> AppResult<Self> {
        let model = cfg.model_for_role(&cfg.derivation_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model.clone())
            .temperature(0.1)
            .timeout_seconds(180)
            .build()
            .map_err(|e| AppError::other(format!("build derivation llm: {e}")))?;
        Ok(Self {
            llm,
            limiter,
            db,
            model,
        })
    }

    #[tracing::instrument(
        level = "info",
        skip(self, formulas),
        fields(agent.role = "derivation", agent.tier = "heavy", formulas = formulas.len())
    )]
    pub async fn chain(
        &self,
        seed_topic: &str,
        formulas: &[Formula],
    ) -> AppResult<DerivationChainNote> {
        let _permit = self.limiter.admit(Role::Derivation).await?;

        let formulas_block = formulas
            .iter()
            .map(|f| {
                format!(
                    "- latex: `{}`\n  symbols: {}\n  source: `{}`\n  context: {}",
                    f.latex,
                    f.symbols.join(", "),
                    f.note_rel_path,
                    f.context_caption
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "{DERIVATION_CHAIN_SKILL}\n\n---\n\n\
             # Seed topic\n{seed_topic}\n\n\
             # Candidate formulas\n{formulas_block}"
        );

        let messages = vec![ChatMessage::user().content(prompt.clone()).build()];
        let schema = StructuredOutputFormat {
            name: "derivation_chain".into(),
            description: Some("Linear derivation chain over candidate formulas".into()),
            schema: Some(derivation_schema()),
            strict: Some(true),
        };

        let started = std::time::Instant::now();
        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("derivation chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty derivation response".into()))?;
        let _ = super::record_agent_call(
            &self.db,
            super::AgentCall {
                role: Role::Derivation,
                input: &prompt,
                output: &text,
                thinking: None,
                payload_json: None,
                research_request_id: None,
                step_index: None,
                phase: Some("llm_call"),
                tool_name: None,
                model_name: None,
                artifact_path: None,
                started,
            },
        )
        .await;
        let parsed: DerivationChainNote = serde_json::from_str(&text).map_err(|e| {
            AppError::Schema(format!("parse derivation: {e} :: {}", truncate(&text, 400)))
        })?;
        debug!(steps = parsed.steps.len(), "derivation ok");
        Ok(parsed)
    }
}
