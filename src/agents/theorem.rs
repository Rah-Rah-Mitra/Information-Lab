//! TheoremProverAgent — heavy tier.
//!
//! Takes a high-confidence Bridge + two Topic snippets + the set of
//! formulas associated with both topics, and produces a formal-style
//! theorem note. Skill text (`skills/theorem_prover.md`) enforces verbatim
//! LaTeX, no invented citations, explicit `(assumption)` tagging for
//! un-discharged steps.
//!
//! One [`Limiter::admit(Role::Theorem)`] gate per call.

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

pub const THEOREM_PROVER_SKILL: &str = include_str!("../../skills/theorem_prover.md");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoremReference {
    pub note_rel_path: String,
    #[serde(default)]
    pub anchor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoremNote {
    pub topic_a: String,
    pub topic_b: String,
    pub bridge_rel_path: String,
    pub title: String,
    pub given: String,
    pub claim: String,
    pub proof_sketch: String,
    pub derivation: Vec<String>,
    pub references: Vec<TheoremReference>,
    pub markdown_body: String,
}

/// Inputs to one theorem-prover call.
#[derive(Debug, Clone, Serialize)]
pub struct TheoremInput {
    pub topic_a: String,
    pub topic_b: String,
    pub bridge_rel_path: String,
    pub bridge_hypothesis: String,
    pub summary_a: String,
    pub summary_b: String,
    pub formulas: Vec<Formula>,
}

fn theorem_schema() -> Value {
    json!({
        "type": "object",
        "required": ["topic_a","topic_b","bridge_rel_path","title","given",
                     "claim","proof_sketch","derivation","references","markdown_body"],
        "properties": {
            "topic_a":         { "type": "string" },
            "topic_b":         { "type": "string" },
            "bridge_rel_path": { "type": "string" },
            "title":           { "type": "string" },
            "given":           { "type": "string" },
            "claim":           { "type": "string" },
            "proof_sketch":    { "type": "string" },
            "derivation":      { "type": "array", "items": { "type": "string" } },
            "references": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["note_rel_path"],
                    "properties": {
                        "note_rel_path": { "type": "string" },
                        "anchor":        { "type": "string" }
                    }
                }
            },
            "markdown_body":   { "type": "string" }
        }
    })
}

#[derive(Clone)]
pub struct TheoremProverAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    db: Db,
    #[allow(dead_code)]
    model: String,
}

impl TheoremProverAgent {
    pub fn new(cfg: &Config, limiter: Arc<Limiter>, db: Db) -> AppResult<Self> {
        let model = cfg.model_for_role(&cfg.theorem_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model.clone())
            .temperature(0.1)
            .timeout_seconds(180)
            .build()
            .map_err(|e| AppError::other(format!("build theorem llm: {e}")))?;
        Ok(Self { llm, limiter, db, model })
    }

    #[tracing::instrument(
        level = "info",
        skip(self, input),
        fields(
            agent.role = "theorem", agent.tier = "heavy",
            topic_a = %input.topic_a, topic_b = %input.topic_b,
            formulas = input.formulas.len()
        )
    )]
    pub async fn prove(&self, input: &TheoremInput) -> AppResult<TheoremNote> {
        let _permit = self.limiter.admit(Role::Theorem).await?;

        let formulas_block = input
            .formulas
            .iter()
            .map(|f| {
                format!(
                    "- `{}` (from `{}`) — {}",
                    f.latex, f.note_rel_path, f.context_caption
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "{THEOREM_PROVER_SKILL}\n\n---\n\n\
             # Bridge\n\
             topic_a: {ta}\n\
             topic_b: {tb}\n\
             bridge_path: {bp}\n\
             hypothesis: {hyp}\n\n\
             # Topic A summary\n{sa}\n\n\
             # Topic B summary\n{sb}\n\n\
             # Linked formulas\n{fb}",
            ta = input.topic_a,
            tb = input.topic_b,
            bp = input.bridge_rel_path,
            hyp = input.bridge_hypothesis,
            sa = input.summary_a,
            sb = input.summary_b,
            fb = formulas_block,
        );

        let messages = vec![ChatMessage::user().content(prompt.clone()).build()];
        let schema = StructuredOutputFormat {
            name: "theorem_note".into(),
            description: Some("Formal-style proof sketch bridging two topics".into()),
            schema: Some(theorem_schema()),
            strict: Some(true),
        };

        let started = std::time::Instant::now();
        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("theorem chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty theorem response".into()))?;
        let _ = super::record_agent_call(
            &self.db,
            super::AgentCall {
                role: Role::Theorem,
                input: &prompt,
                output: &text,
                thinking: None,
                payload_json: None,
                started,
            },
        )
        .await;
        let parsed: TheoremNote = serde_json::from_str(&text).map_err(|e| {
            AppError::Schema(format!("parse theorem: {e} :: {}", truncate(&text, 400)))
        })?;
        debug!(steps = parsed.derivation.len(), "theorem ok");
        Ok(parsed)
    }
}
