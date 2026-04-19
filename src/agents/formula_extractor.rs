//! FormulaExtractorAgent — light tier.
//!
//! Salvage pass over math-dense chunks that the reasoner's text-only view
//! loses or garbles. `pdf_oxide` gives reading-order text, but equations
//! often arrive as stray Unicode glyph runs (`∑` without indices, `\alpha`
//! dropped to `α`, display blocks flattened to inline). This agent takes
//! a single chunk's raw text, recovers its LaTeX, and upserts each formula
//! into the shared `formulas` corpus.
//!
//! The companion module [`crate::formula_detect`] decides which chunks
//! deserve this pass — `math_density_score ≥ formula_detect_tau`. Low-
//! scoring chunks never hit the LLM.
//!
//! One [`Limiter::admit(Role::FormulaExtractor)`] gate per call.

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
    error::{AppError, AppResult},
    limiter::{Limiter, Role},
};

use super::truncate;

pub const FORMULA_EXTRACTOR_SKILL: &str =
    include_str!("../../skills/formula_extractor.md");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedFormula {
    pub latex: String,
    #[serde(default)]
    pub symbols: Vec<String>,
    #[serde(default)]
    pub context_caption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaExtractOutput {
    pub formulas: Vec<ExtractedFormula>,
}

fn formula_schema() -> Value {
    json!({
        "type": "object",
        "required": ["formulas"],
        "properties": {
            "formulas": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["latex"],
                    "properties": {
                        "latex":           { "type": "string" },
                        "symbols":         { "type": "array", "items": { "type": "string" } },
                        "context_caption": { "type": "string" }
                    }
                }
            }
        }
    })
}

#[derive(Clone)]
pub struct FormulaExtractorAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    #[allow(dead_code)]
    model: String,
}

impl FormulaExtractorAgent {
    pub fn new(cfg: &Config, limiter: Arc<Limiter>) -> AppResult<Self> {
        // Light-tier default (Gemma 4 26B); honour override.
        let model = cfg.model_for_override(&cfg.formula_model, &cfg.light_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model.clone())
            .temperature(0.0)
            .timeout_seconds(90)
            .build()
            .map_err(|e| AppError::other(format!("build formula llm: {e}")))?;
        Ok(Self { llm, limiter, model })
    }

    #[tracing::instrument(
        level = "info",
        skip(self, chunk_text),
        fields(agent.role = "formula_extractor", agent.tier = "light", bytes = chunk_text.len())
    )]
    pub async fn extract(
        &self,
        chunk_text: &str,
    ) -> AppResult<FormulaExtractOutput> {
        let _permit = self.limiter.admit(Role::FormulaExtractor).await?;

        // Hard cap the prompt — the light tier's context is tight and the
        // heuristic has already told us the density is high.
        let body = if chunk_text.len() > 6000 {
            let mut cut = 6000;
            while !chunk_text.is_char_boundary(cut) && cut > 0 {
                cut -= 1;
            }
            &chunk_text[..cut]
        } else {
            chunk_text
        };

        let prompt = format!(
            "{FORMULA_EXTRACTOR_SKILL}\n\n---\n\n# Chunk text\n\n{body}"
        );

        let messages = vec![ChatMessage::user().content(prompt).build()];
        let schema = StructuredOutputFormat {
            name: "formula_extract".into(),
            description: Some("Salvaged LaTeX formulas from a math-dense chunk".into()),
            schema: Some(formula_schema()),
            strict: Some(true),
        };

        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("formula chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty formula response".into()))?;
        let parsed: FormulaExtractOutput = serde_json::from_str(&text).map_err(|e| {
            AppError::Schema(format!(
                "parse formula: {e} :: {}",
                truncate(&text, 400)
            ))
        })?;
        debug!(count = parsed.formulas.len(), "formula extract ok");
        Ok(parsed)
    }
}
