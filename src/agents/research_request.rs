use std::sync::Arc;

use autoagents::llm::{
    backends::google::Google,
    builder::LLMBuilder,
    chat::{ChatMessage, ChatProvider, StructuredOutputFormat},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    config::Config,
    db::Db,
    error::{AppError, AppResult},
    limiter::{Limiter, Role},
};

use super::truncate;

pub const RESEARCH_REQUEST_SKILL: &str = include_str!("../../skills/research_request.md");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub title: String,
    pub summary: String,
    pub markdown_body: String,
    #[serde(default)]
    pub references: Vec<String>,
    #[serde(default)]
    pub open_questions: Vec<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResearchContext {
    pub problem: String,
    pub max_iterations: u8,
    pub skills_scope: Vec<String>,
    pub topic_context: Vec<String>,
    pub formula_context: Vec<String>,
    pub iteration_index: u8,
    pub prior_report: Option<String>,
}

fn research_result_schema() -> Value {
    json!({
        "type": "object",
        "required": ["title", "summary", "markdown_body", "confidence"],
        "properties": {
            "title": { "type": "string" },
            "summary": { "type": "string" },
            "markdown_body": { "type": "string" },
            "references": { "type": "array", "items": { "type": "string" } },
            "open_questions": { "type": "array", "items": { "type": "string" } },
            "confidence": { "type": "number" }
        }
    })
}

#[derive(Clone)]
pub struct ResearchRequestAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    db: Db,
}

impl ResearchRequestAgent {
    pub fn new(cfg: &Config, limiter: Arc<Limiter>, db: Db) -> AppResult<Self> {
        let model = cfg.model_for_role(&cfg.report_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model)
            .temperature(0.2)
            .timeout_seconds(180)
            .build()
            .map_err(|e| AppError::other(format!("build ad-hoc research llm: {e}")))?;
        Ok(Self { llm, limiter, db })
    }

    pub async fn run(&self, ctx: &ResearchContext) -> AppResult<ResearchResult> {
        let _permit = self.limiter.admit(Role::Report).await?;

        let prompt = format!(
            "{RESEARCH_REQUEST_SKILL}\n\n---\n\n# Problem\n{problem}\n\n# Iteration\n{iter}/{iters}\n\n# Skills scope\n{scope}\n\n# Topic context\n{topics}\n\n# Formula context\n{formulas}\n\n# Prior draft\n{prior}",
            problem = ctx.problem,
            iter = ctx.iteration_index,
            iters = ctx.max_iterations,
            scope = if ctx.skills_scope.is_empty() {
                "(none)".to_string()
            } else {
                ctx.skills_scope.join(", ")
            },
            topics = if ctx.topic_context.is_empty() {
                "(none)".to_string()
            } else {
                ctx.topic_context.join("\n\n")
            },
            formulas = if ctx.formula_context.is_empty() {
                "(none)".to_string()
            } else {
                ctx.formula_context.join("\n")
            },
            prior = ctx.prior_report.as_deref().unwrap_or("(none)")
        );

        let messages = vec![ChatMessage::user().content(prompt.clone()).build()];
        let schema = StructuredOutputFormat {
            name: "research_result".into(),
            description: Some("Ad-hoc research request response".into()),
            schema: Some(research_result_schema()),
            strict: Some(true),
        };

        let started = std::time::Instant::now();
        let resp = self
            .llm
            .chat(&messages, Some(schema.clone()))
            .await
            .map_err(|e| AppError::other(format!("ad-hoc research chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty ad-hoc research response".into()))?;
        let mut parsed = parse_research_result(&text);
        let _ = super::record_agent_call(
            &self.db,
            super::AgentCall {
                role: Role::Report,
                input: &prompt,
                output: &text,
                thinking: None,
                payload_json: None,
                research_request_id: None,
                step_index: None,
                phase: None,
                tool_name: None,
                model_name: None,
                artifact_path: None,
                started,
            },
        )
        .await;
        if parsed.is_ok() {
            return parsed;
        }

        // Some provider/model combinations ignore strict JSON schema or return
        // fenced JSON. Retry once with non-strict output and salvage JSON from
        // prose if needed.
        let relaxed = StructuredOutputFormat {
            strict: Some(false),
            ..schema
        };
        let retry_resp = self
            .llm
            .chat(&messages, Some(relaxed))
            .await
            .map_err(|e| AppError::other(format!("ad-hoc research chat retry: {e}")))?;
        let retry_text = retry_resp
            .text()
            .ok_or_else(|| AppError::Schema("empty ad-hoc research retry response".into()))?;
        let _ = super::record_agent_call(
            &self.db,
            super::AgentCall {
                role: Role::Report,
                input: &prompt,
                output: &retry_text,
                thinking: None,
                payload_json: None,
                research_request_id: None,
                step_index: None,
                phase: Some("llm_call_retry"),
                tool_name: None,
                model_name: None,
                artifact_path: None,
                started: std::time::Instant::now(),
            },
        )
        .await;

        parsed = parse_research_result(&retry_text);
        parsed
    }
}

fn parse_research_result(text: &str) -> AppResult<ResearchResult> {
    if let Ok(parsed) = serde_json::from_str::<ResearchResult>(text) {
        return Ok(parsed);
    }
    if let Some(json_blob) = extract_first_json_object(text) {
        return serde_json::from_str::<ResearchResult>(&json_blob).map_err(|e| {
            AppError::Schema(format!(
                "parse ad-hoc research (extracted json): {e} :: {}",
                truncate(text, 400)
            ))
        });
    }
    Err(AppError::Schema(format!(
        "parse ad-hoc research: no valid json object found :: {}",
        truncate(text, 400)
    )))
}

fn extract_first_json_object(text: &str) -> Option<String> {
    let mut start = None;
    let mut depth = 0_i32;
    let mut in_str = false;
    let mut escape = false;
    let chars: Vec<char> = text.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if in_str {
            if escape {
                escape = false;
                continue;
            }
            if *c == '\\' {
                escape = true;
                continue;
            }
            if *c == '"' {
                in_str = false;
            }
            continue;
        }

        match c {
            '"' => in_str = true,
            '{' => {
                if depth == 0 {
                    start = Some(i);
                }
                depth += 1;
            }
            '}' => {
                if depth > 0 {
                    depth -= 1;
                    if depth == 0 {
                        if let Some(s) = start {
                            let candidate: String = chars[s..=i].iter().collect();
                            if serde_json::from_str::<serde_json::Value>(&candidate).is_ok() {
                                return Some(candidate);
                            }
                        }
                        start = None;
                    }
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::extract_first_json_object;

    #[test]
    fn extracts_json_from_fenced_output() {
        let s = "```json\n{\"title\":\"t\",\"summary\":\"s\",\"markdown_body\":\"b\",\"confidence\":0.9}\n```";
        let got = extract_first_json_object(s).expect("json object");
        assert!(got.contains("\"title\":\"t\""));
    }
}
