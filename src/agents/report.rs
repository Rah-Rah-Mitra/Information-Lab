//! ReportWriterAgent — heavy tier, LOW thinking.
//!
//! Daily cadence. Consumes summaries of the previous day's Syntheses,
//! Bridges, and Theorems; emits one prose note per day under
//! `Generated/_Reports/{YYYY-MM-DD}.md`.

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

pub const REPORT_WRITER_SKILL: &str = include_str!("../../skills/report_writer.md");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    pub heading: String,
    pub body: String,
    pub cited_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReport {
    pub date: String,
    pub headline: String,
    pub sections: Vec<ReportSection>,
    pub markdown_body: String,
}

/// One input row — a thin summary of a note produced in the window.
#[derive(Debug, Clone, Serialize)]
pub struct ReportInput {
    pub kind: String, // "synthesis" | "bridge" | "theorem"
    pub title: String,
    pub summary: String,
    pub note_rel_path: String,
}

fn report_schema() -> Value {
    json!({
        "type": "object",
        "required": ["date","headline","sections","markdown_body"],
        "properties": {
            "date":     { "type": "string" },
            "headline": { "type": "string" },
            "sections": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["heading","body","cited_notes"],
                    "properties": {
                        "heading":     { "type": "string" },
                        "body":        { "type": "string" },
                        "cited_notes": { "type": "array", "items": { "type": "string" } }
                    }
                }
            },
            "markdown_body": { "type": "string" }
        }
    })
}

#[derive(Clone)]
pub struct ReportWriterAgent {
    llm: Arc<Google>,
    limiter: Arc<Limiter>,
    db: Db,
    #[allow(dead_code)]
    model: String,
}

impl ReportWriterAgent {
    pub fn new(cfg: &Config, limiter: Arc<Limiter>, db: Db) -> AppResult<Self> {
        let model = cfg.model_for_role(&cfg.report_model);
        let llm: Arc<Google> = LLMBuilder::<Google>::new()
            .api_key(cfg.api_key.clone())
            .model(model.clone())
            .temperature(0.3)
            .timeout_seconds(180)
            .build()
            .map_err(|e| AppError::other(format!("build report llm: {e}")))?;
        Ok(Self { llm, limiter, db, model })
    }

    #[tracing::instrument(
        level = "info",
        skip(self, inputs),
        fields(agent.role = "report", agent.tier = "heavy", date = %date, items = inputs.len())
    )]
    pub async fn write(
        &self,
        date: &str,
        inputs: &[ReportInput],
    ) -> AppResult<DailyReport> {
        let _permit = self.limiter.admit(Role::Report).await?;

        let inputs_block = inputs
            .iter()
            .map(|r| {
                format!(
                    "- [{kind}] **{title}** ({path}) — {summary}",
                    kind = r.kind,
                    title = r.title,
                    path = r.note_rel_path,
                    summary = r.summary,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "{REPORT_WRITER_SKILL}\n\n---\n\n\
             # Date\n{date}\n\n\
             # Items produced in the last 24h\n{inputs_block}"
        );

        let messages = vec![ChatMessage::user().content(prompt.clone()).build()];
        let schema = StructuredOutputFormat {
            name: "daily_report".into(),
            description: Some("Prose report summarising the day's research output".into()),
            schema: Some(report_schema()),
            strict: Some(true),
        };

        let started = std::time::Instant::now();
        let resp = self
            .llm
            .chat(&messages, Some(schema))
            .await
            .map_err(|e| AppError::other(format!("report chat: {e}")))?;
        let text = resp
            .text()
            .ok_or_else(|| AppError::Schema("empty report response".into()))?;
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
        let parsed: DailyReport = serde_json::from_str(&text).map_err(|e| {
            AppError::Schema(format!("parse report: {e} :: {}", truncate(&text, 400)))
        })?;
        debug!(sections = parsed.sections.len(), "report ok");
        Ok(parsed)
    }
}
