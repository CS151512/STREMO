mod api;
mod infrastructure;
mod models;
mod repository;
mod service;

use axum::{routing::get, Router};
use infrastructure::grpc_client::ModerationClient;
use repository::{postgres::PostgresBatcher, redis_pubsub::RedisPubSub};
use service::manager::ChatManager;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Chat Service...");

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let mod_grpc_url = std::env::var("MODERATION_URL").unwrap_or_else(|_| "http://moderation-service:50051".to_string());
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost/stremo".to_string());
    let redis_pubsub = RedisPubSub::new(&redis_url)?;
    let db_batcher = PostgresBatcher::new(&db_url).await?;
    let mod_client = ModerationClient::connect(mod_grpc_url).await?;
    let chat_manager = Arc::new(ChatManager::new(redis_pubsub, db_batcher, mod_client));
    let api_router = api::http::routes::create_router(chat_manager.clone());

    let app = Router::new()
        .route("/ws/chat/:channel_id", get(api::ws::ws_handler))
        .with_state(chat_manager)
        .merge(api_router)
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Chat Service WebSocket Server listening on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
