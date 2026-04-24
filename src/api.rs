//! Lightweight read-only research timeline API.

use std::net::SocketAddr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tracing::{error, info};

use crate::db::{AgentEventRow, Db, ResearchSummary};

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
