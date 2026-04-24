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
        "max_iterations": body.max_iterations.unwrap_or(2),
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
