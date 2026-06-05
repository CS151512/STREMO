use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::api::ws::connection::handle_ws_connection;
use crate::service::manager::AnalyticsManager;

pub fn create_router(manager: AnalyticsManager) -> Router {
    let state = Arc::new(manager);

    Router::new()
        .route("/health", get(health_check))
        .route(
            "/api/v1/analytics/:stream_id/summary",
            get(get_stream_summary),
        )
        .route("/api/v1/analytics/:stream_id/live", get(ws_handler))
        .with_state(state)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}

async fn get_stream_summary(
    Path(stream_id): Path<String>,
    State(manager): State<Arc<AnalyticsManager>>,
) -> impl IntoResponse {
    match manager.get_summary(&stream_id).await {
        Ok(summary) => (StatusCode::OK, Json(json!(summary))),
        Err(e) => {
            tracing::error!("Error getting summary for {}: {}", stream_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch analytics"})),
            )
        }
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(stream_id): Path<String>,
    State(manager): State<Arc<AnalyticsManager>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws_connection(socket, stream_id, manager.ws_hub.clone()))
}
