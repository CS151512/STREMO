use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;

use crate::models::dto::{ChatHistoryQuery, ChatHistoryResponse};
use crate::service::manager::ChatManager;

pub fn create_router(manager: Arc<ChatManager>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/chat/:channel_id/history", get(get_chat_history))
        .with_state(manager)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "healthy"})))
}

async fn get_chat_history(
    Path(channel_id): Path<String>,
    Query(query): Query<ChatHistoryQuery>,
    State(manager): State<Arc<ChatManager>>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).max(0);

    match manager.get_history(&channel_id, limit, offset).await {
        Ok(messages) => (StatusCode::OK, Json(ChatHistoryResponse { messages })).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch chat history for channel {}: {}", channel_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to fetch chat history"})),
            )
                .into_response()
        }
    }
}
