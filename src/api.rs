use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::trace::TraceLayer;

use crate::{
    config::Config,
    db::{AgentTaskKind, Db},
};

#[derive(Clone)]
pub struct ApiState {
    pub db: Db,
    pub cfg: Arc<Config>,
}

#[derive(Debug, Deserialize)]
pub struct ResearchRequest {
    pub problem: String,
    pub max_iterations: Option<u8>,
    pub skills_scope: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ResearchEnqueueResponse {
    pub id: i64,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct ResearchStatusResponse {
    pub id: i64,
    pub state: String,
    pub error: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/research", post(post_research))
        .route("/research/{id}", get(get_research_status))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn post_research(
    State(state): State<ApiState>,
    Json(req): Json<ResearchRequest>,
) -> Result<(StatusCode, Json<ResearchEnqueueResponse>), (StatusCode, Json<serde_json::Value>)> {
    if req.problem.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "problem is required"})),
        ));
    }

    let payload = json!({
        "problem": req.problem,
        "max_iterations": req.max_iterations.unwrap_or(2),
        "skills_scope": req.skills_scope.unwrap_or_default(),
        "requested_by": "api",
        "bind_addr": state.cfg.http_bind,
    });

    let id = state
        .db
        .enqueue_agent_task(AgentTaskKind::ResearchRequest, &payload)
        .await
        .map_err(internal_err)?;

    Ok((
        StatusCode::ACCEPTED,
        Json(ResearchEnqueueResponse {
            id,
            state: "queued".to_string(),
        }),
    ))
}

async fn get_research_status(
    State(state): State<ApiState>,
    Path(id): Path<i64>,
) -> Result<Json<ResearchStatusResponse>, (StatusCode, Json<serde_json::Value>)> {
    let row = state
        .db
        .get_agent_task(id)
        .await
        .map_err(internal_err)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "research request not found"})),
            )
        })?;

    if row.kind != AgentTaskKind::ResearchRequest.as_str() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "research request not found"})),
        ));
    }

    Ok(Json(ResearchStatusResponse {
        id: row.id,
        state: map_state(&row.state),
        error: row.last_error,
        created_at: row.created_at,
        completed_at: row.completed_at,
    }))
}

fn map_state(state: &str) -> String {
    match state {
        "pending" => "queued".to_string(),
        "running" => "running".to_string(),
        "done" => "done".to_string(),
        "error" => "error".to_string(),
        other => other.to_string(),
    }
}

fn internal_err(e: impl std::fmt::Display) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": e.to_string()})),
    )
}
