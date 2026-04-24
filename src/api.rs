//! Minimal HTTP endpoint to enqueue independent research tasks.

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::db::{AgentTaskKind, Db};

#[derive(Clone)]
struct ApiState {
    db: Db,
}

#[derive(Debug, Deserialize)]
pub struct ResearchRequest {
    pub query: String,
    #[serde(default)]
    pub max_web_queries: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct ResearchQueued {
    pub task_id: i64,
    pub kind: &'static str,
}

pub async fn spawn(db: Db, bind: String) {
    let state = Arc::new(ApiState { db });
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/research/request", post(enqueue_research))
        .with_state(state);

    match TcpListener::bind(&bind).await {
        Ok(listener) => {
            info!(bind = %bind, "research api listening");
            tokio::spawn(async move {
                if let Err(e) = axum::serve(listener, app).await {
                    error!(error = %e, "research api server failed");
                }
            });
        }
        Err(e) => {
            error!(bind = %bind, error = %e, "research api bind failed");
        }
    }
}

async fn healthz() -> &'static str {
    "ok"
}

async fn enqueue_research(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<ResearchRequest>,
) -> Result<Json<ResearchQueued>, (StatusCode, String)> {
    let payload = json!({
        "query": req.query,
        "max_web_queries": req.max_web_queries.unwrap_or(2).clamp(1, 4),
    });

    let id = state
        .db
        .enqueue_agent_task(AgentTaskKind::Research, &payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ResearchQueued {
        task_id: id,
        kind: "Research",
    }))
}
