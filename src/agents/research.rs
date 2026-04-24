//! ResearchSolverAgent — independent heavy-tier loop for ad-hoc research queries.

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

pub const RESEARCH_SOLVER_SKILL: &str = include_str!("../../skills/research_solver.md");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub title: String,
    pub summary: String,
    pub markdown_body: String,
    #[serde(default)]
    pub references: Vec<String>,
    #[serde(default)]
    pub open_questions: Vec<String>,
}

fn schema() -> Value {
    json!({
        "type": "object",
        "required": ["title", "summary", "markdown_body", "references", "open_questions"],
        "properties": {
            "title": { "type": "string" },
            "summary": { "type": "string" },
            "markdown_body": { "type": "string" },
            "references": { "type": "array", "items": { "type": "string" } },
            "open_questions": { "type": "array", "items": { "type": "string" } }
        }
    })
}

#[derive(Clone)]
pub struct ResearchSolverAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    db: Db,
}

impl ResearchSolverAgent {
    pub fn new(cfg: &Config, limiter: Arc<Limiter>, db: Db) -> AppResult<Self> {
        let model = cfg.model_for_role(&cfg.report_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model)
            .temperature(0.2)
            .timeout_seconds(240)
            .build()
            .map_err(|e| AppError::other(format!("build research llm: {e}")))?;
        Ok(Self { llm, limiter, db })
    }

    #[tracing::instrument(level = "info", skip(self, context, web_findings), fields(agent.role = "research", q = %query))]
    pub async fn solve(
        &self,
        query: &str,
        context: &str,
        web_findings: &str,
    ) -> AppResult<ResearchResult> {
        let _permit = self.limiter.admit(Role::Report).await?;
        let prompt = format!(
            "{RESEARCH_SOLVER_SKILL}\n\n---\n\n# Research request\n{query}\n\n# Local vault context\n{context}\n\n# Web findings (Tavily)\n{web_findings}"
        );

        let messages = vec![ChatMessage::user().content(prompt.clone()).build()];
        let schema = StructuredOutputFormat {
            name: "research_result".into(),
            description: Some("Research answer with equations, references, and open questions".into()),
            schema: Some(schema()),
            strict: Some(true),
        };

        let started = std::time::Instant::now();
        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("research chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty research response".into()))?;

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

        let parsed: ResearchResult = serde_json::from_str(&text).map_err(|e| {
            AppError::Schema(format!("parse research response: {e} :: {}", truncate(&text, 400)))
        })?;
        debug!(refs = parsed.references.len(), "research solved");
        Ok(parsed)
    }
}
