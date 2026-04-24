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

use std::{collections::HashSet, sync::Arc};

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

use super::{scrub_llm_text, truncate};

pub const FORMULA_EXTRACTOR_SKILL: &str = include_str!("../../skills/formula_extractor.md");

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
    db: Db,
    #[allow(dead_code)]
    model: String,
}

impl FormulaExtractorAgent {
    const MAX_REGEN_PASSES: u8 = 3;

    pub fn new(cfg: &Config, limiter: Arc<Limiter>, db: Db) -> AppResult<Self> {
        // Light-tier default (Gemma 4 26B); honour override.
        let model = cfg.model_for_override(&cfg.formula_model, &cfg.light_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model.clone())
            .temperature(0.0)
            .timeout_seconds(90)
            .build()
            .map_err(|e| AppError::other(format!("build formula llm: {e}")))?;
        Ok(Self {
            llm,
            limiter,
            db,
            model,
        })
    }

    #[tracing::instrument(
        level = "info",
        skip(self, chunk_text),
        fields(agent.role = "formula_extractor", agent.tier = "light", bytes = chunk_text.len())
    )]
    pub async fn extract(&self, chunk_text: &str) -> AppResult<FormulaExtractOutput> {
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

        let mut accepted: Vec<ExtractedFormula> = Vec::new();
        let mut seen_latex = HashSet::new();
        let mut failed_render: Vec<String> = Vec::new();

        for pass in 1..=Self::MAX_REGEN_PASSES {
            let regen_context = if failed_render.is_empty() {
                String::new()
            } else {
                format!(
                    "\n\n# Failed render formulas from previous pass\n{}\n\n\
                     Regenerate only corrected LaTeX for these formulas. Keep formulas that already render unchanged.",
                    failed_render
                        .iter()
                        .map(|f| format!("- `{f}`"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            };
            let prompt = format!(
                "{FORMULA_EXTRACTOR_SKILL}\n\n---\n\n# Pass\n{pass}/{max}\n\n# Chunk text\n\n{body}{regen_context}",
                max = Self::MAX_REGEN_PASSES
            );

            let messages = vec![ChatMessage::user().content(prompt.clone()).build()];
            let schema = StructuredOutputFormat {
                name: "formula_extract".into(),
                description: Some("Salvaged LaTeX formulas from a math-dense chunk".into()),
                schema: Some(formula_schema()),
                strict: Some(true),
            };

            let started = std::time::Instant::now();
            let resp = self
                .llm
                .chat(&messages, Some(schema))
                .await
                .map_err(|e| AppError::other(format!("formula chat: {e}")))?;
            let text = resp
                .text()
                .ok_or_else(|| AppError::Schema("empty formula response".into()))?;
            let _ = super::record_agent_call(
                &self.db,
                super::AgentCall {
                    role: Role::FormulaExtractor,
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
            let mut parsed: FormulaExtractOutput = serde_json::from_str(&text).map_err(|e| {
                AppError::Schema(format!("parse formula: {e} :: {}", truncate(&text, 400)))
            })?;
            parsed.formulas.retain_mut(|f| {
                f.latex = sanitize_latex(&f.latex);
                f.context_caption = sanitize_text(&f.context_caption);
                f.symbols = f
                    .symbols
                    .iter()
                    .map(|s| sanitize_text(s))
                    .filter(|s| !s.is_empty())
                    .collect();
                !f.latex.is_empty()
            });

            failed_render.clear();
            for f in parsed.formulas {
                if !quick_formula_render_scan(&f.latex) {
                    failed_render.push(f.latex);
                    continue;
                }
                if seen_latex.insert(f.latex.clone()) {
                    accepted.push(f);
                }
            }
            if failed_render.is_empty() {
                break;
            }
        }

        debug!(
            count = accepted.len(),
            failed_render = failed_render.len(),
            "formula extract ok"
        );
        Ok(FormulaExtractOutput { formulas: accepted })
    }
}

/// Strip NUL / chat-template leakage and collapse whitespace runs that a
/// stripped token sometimes leaves behind.
fn sanitize_latex(s: &str) -> String {
    let cleaned = scrub_llm_text(s);
    let mut out = String::with_capacity(cleaned.len());
    let mut prev_space = false;
    for ch in cleaned.chars() {
        let is_space = ch == ' ' || ch == '\t';
        if is_space && prev_space {
            continue;
        }
        prev_space = is_space;
        out.push(ch);
    }
    out.trim().to_string()
}

fn sanitize_text(s: &str) -> String {
    scrub_llm_text(s)
}

/// Lightweight pre-render scan for obvious LaTeX breakage that tends to fail
/// markdown math renderers (KaTeX/MathJax): unmatched braces/delimiters,
/// dangling escapes, and malformed `\left`/`\right` pairing.
fn quick_formula_render_scan(latex: &str) -> bool {
    if latex.trim().is_empty() {
        return false;
    }
    if latex.contains('\n') || latex.contains('\r') || latex.contains('\u{0000}') {
        return false;
    }

    let mut braces = 0i32;
    let mut parens = 0i32;
    let mut brackets = 0i32;
    let mut left_count = 0usize;
    let mut right_count = 0usize;
    let mut chars = latex.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.peek() {
                Some('\\') => {
                    chars.next();
                    continue;
                }
                None => return false,
                _ => {}
            }
            let mut cmd = String::new();
            while let Some(c) = chars.peek().copied() {
                if c.is_ascii_alphabetic() {
                    cmd.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            if cmd == "left" {
                left_count += 1;
            } else if cmd == "right" {
                right_count += 1;
            }
            continue;
        }
        match ch {
            '{' => braces += 1,
            '}' => {
                braces -= 1;
                if braces < 0 {
                    return false;
                }
            }
            '(' => parens += 1,
            ')' => {
                parens -= 1;
                if parens < 0 {
                    return false;
                }
            }
            '[' => brackets += 1,
            ']' => {
                brackets -= 1;
                if brackets < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    braces == 0 && parens == 0 && brackets == 0 && left_count == right_count
}

#[cfg(test)]
mod tests {
    use super::quick_formula_render_scan;

    #[test]
    fn formula_scan_accepts_balanced_latex() {
        assert!(quick_formula_render_scan(r"\frac{a+b}{c+d}"));
        assert!(quick_formula_render_scan(r"\left( x + y \right)^2"));
    }

    #[test]
    fn formula_scan_rejects_unbalanced_or_dangling_input() {
        assert!(!quick_formula_render_scan(r"\frac{a+b}{c+d"));
        assert!(!quick_formula_render_scan(r"\left( x + y )"));
        assert!(!quick_formula_render_scan("x + y \\"));
    }
}
