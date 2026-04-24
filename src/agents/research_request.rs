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
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResearchContext {
    pub problem: String,
    pub max_iterations: u8,
    pub skills_scope: Vec<String>,
    pub topic_context: Vec<String>,
    pub formula_context: Vec<String>,
}

fn research_result_schema() -> Value {
    json!({
        "type": "object",
        "required": ["title", "summary", "markdown_body", "confidence"],
        "properties": {
            "title": { "type": "string" },
            "summary": { "type": "string" },
            "markdown_body": { "type": "string" },
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
            "{RESEARCH_REQUEST_SKILL}\n\n---\n\n# Problem\n{problem}\n\n# Max iterations\n{iters}\n\n# Skills scope\n{scope}\n\n# Topic context\n{topics}\n\n# Formula context\n{formulas}",
            problem = ctx.problem,
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
            }
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
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("ad-hoc research chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty ad-hoc research response".into()))?;
        let _ = super::record_agent_call(
            &self.db,
            super::AgentCall {
                role: Role::Report,
                input: &prompt,
                output: &text,
                thinking: None,
                payload_json: None,
                started,
            },
        )
        .await;

        serde_json::from_str::<ResearchResult>(&text).map_err(|e| {
            AppError::Schema(format!(
                "parse ad-hoc research: {e} :: {}",
                truncate(&text, 400)
            ))
        })
    }
}
