//! Lightweight read-only research timeline API.

use std::net::SocketAddr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{error, info};

use crate::db::{AgentEventRow, AgentTaskKind, Db, ResearchSummary};

#[derive(Clone)]
struct ApiState {
    db: Db,
}

#[derive(Debug, Serialize)]
struct ResearchEnvelope {
    summary: ResearchSummary,
    events: Vec<AgentEventRow>,
}

pub fn spawn(db: Db, bind: String) {
    tokio::spawn(async move {
        let app = Router::new()
            .route("/research/{id}", get(get_research))
            .route("/research/{id}/events", get(get_research_events))
            .route("/research/request", post(post_research_request))
            .route("/monitor/executions", get(get_execution_index))
            .route("/monitor/executions/{id}", get(get_execution_monitor))
            .with_state(ApiState { db });

        let Ok(addr): Result<SocketAddr, _> = bind.parse() else {
            error!(%bind, "invalid RESEARCH_API_BIND; api disabled");
            return;
        };

        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                info!(%addr, "research api listening");
                if let Err(e) = axum::serve(listener, app).await {
                    error!(error = %e, "research api server exited");
                }
            }
            Err(e) => {
                error!(error = %e, %addr, "failed to bind research api");
            }
        }
    });
}

#[derive(Debug, Deserialize)]
struct ResearchRequestBody {
    problem: String,
    #[serde(default)]
    max_iterations: Option<u8>,
    #[serde(default)]
    skills_scope: Vec<String>,
    #[serde(default)]
    telegram_chat_id: Option<String>,
    #[serde(default)]
    telegram_message_id: Option<i64>,
}

#[derive(Debug, Serialize)]
struct ResearchRequestAccepted {
    task_id: i64,
    status: &'static str,
}

#[derive(Debug, Deserialize)]
struct ExecutionIndexQuery {
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Serialize)]
struct ExecutionMonitorEnvelope {
    summary: ResearchSummary,
    totals: ExecutionTotals,
    by_agent_role: BTreeMap<String, i64>,
    by_phase: BTreeMap<String, i64>,
    by_event_kind: BTreeMap<String, i64>,
    by_model: BTreeMap<String, i64>,
    by_tool: BTreeMap<String, i64>,
    step_span: Option<StepSpan>,
    events: Vec<AgentEventRow>,
}

#[derive(Debug, Serialize)]
struct ExecutionTotals {
    event_count: usize,
    tokens_sent: i64,
    tokens_received: i64,
    duration_ms_total: i64,
}

#[derive(Debug, Serialize)]
struct StepSpan {
    min: i64,
    max: i64,
}

async fn post_research_request(
    State(state): State<ApiState>,
    Json(body): Json<ResearchRequestBody>,
) -> impl IntoResponse {
    if body.problem.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error":"problem must be non-empty"})),
        )
            .into_response();
    }
    let payload = serde_json::json!({
        "problem": body.problem,
        "max_iterations": body.max_iterations.unwrap_or(8),
        "skills_scope": body.skills_scope,
        "telegram": {
            "chat_id": body.telegram_chat_id,
            "message_id": body.telegram_message_id
        }
    });
    match state
        .db
        .enqueue_agent_task(AgentTaskKind::Research, &payload)
        .await
    {
        Ok(task_id) => (
            StatusCode::ACCEPTED,
            Json(ResearchRequestAccepted {
                task_id,
                status: "queued",
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn get_research_events(
    Path(id): Path<String>,
    State(state): State<ApiState>,
) -> impl IntoResponse {
    match state.db.list_research_events(&id).await {
        Ok(events) => (StatusCode::OK, Json(events)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn get_research(Path(id): Path<String>, State(state): State<ApiState>) -> impl IntoResponse {
    match state.db.get_research_summary(&id).await {
        Ok(Some(summary)) => {
            let events = state.db.list_research_events(&id).await.unwrap_or_default();
            (StatusCode::OK, Json(ResearchEnvelope { summary, events })).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error":"research request not found"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn get_execution_index(
    State(state): State<ApiState>,
    axum::extract::Query(query): axum::extract::Query<ExecutionIndexQuery>,
) -> impl IntoResponse {
    let limit = query.limit.clamp(1, 200);
    match state.db.list_research_summaries(limit).await {
        Ok(rows) => (StatusCode::OK, Json(rows)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn get_execution_monitor(
    Path(id): Path<String>,
    State(state): State<ApiState>,
) -> impl IntoResponse {
    let summary = match state.db.get_research_summary(&id).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error":"research request not found"})),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response();
        }
    };

    let events = match state.db.list_research_events(&id).await {
        Ok(e) => e,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response();
        }
    };

    let mut by_agent_role = BTreeMap::new();
    let mut by_phase = BTreeMap::new();
    let mut by_event_kind = BTreeMap::new();
    let mut by_model = BTreeMap::new();
    let mut by_tool = BTreeMap::new();
    let mut tokens_sent = 0_i64;
    let mut tokens_received = 0_i64;
    let mut duration_ms_total = 0_i64;
    let mut min_step: Option<i64> = None;
    let mut max_step: Option<i64> = None;

    for event in &events {
        *by_agent_role.entry(event.agent_role.clone()).or_insert(0) += 1;
        *by_event_kind.entry(event.event_kind.clone()).or_insert(0) += 1;
        if let Some(phase) = &event.phase {
            *by_phase.entry(phase.clone()).or_insert(0) += 1;
        }
        if let Some(model) = &event.model_name {
            *by_model.entry(model.clone()).or_insert(0) += 1;
        }
        if let Some(tool) = &event.tool_name {
            *by_tool.entry(tool.clone()).or_insert(0) += 1;
        }
        tokens_sent += event.tokens_sent;
        tokens_received += event.tokens_received;
        duration_ms_total += event.duration_ms.unwrap_or(0);
        if let Some(step) = event.step_index {
            min_step = Some(min_step.map_or(step, |v| v.min(step)));
            max_step = Some(max_step.map_or(step, |v| v.max(step)));
        }
    }

    let step_span = match (min_step, max_step) {
        (Some(min), Some(max)) => Some(StepSpan { min, max }),
        _ => None,
    };

    (
        StatusCode::OK,
        Json(ExecutionMonitorEnvelope {
            summary,
            totals: ExecutionTotals {
                event_count: events.len(),
                tokens_sent,
                tokens_received,
                duration_ms_total,
            },
            by_agent_role,
            by_phase,
            by_event_kind,
            by_model,
            by_tool,
            step_span,
            events,
        }),
    )
        .into_response()
}
