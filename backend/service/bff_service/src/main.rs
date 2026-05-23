use axum::Router;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod grpc_clients;
mod middleware;
mod models;
mod utils;

pub use utils::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,bff_service=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting BFF Service (API Gateway)...");

    let config = config::Config::from_env();
    let manager = RedisConnectionManager::new(config.redis_url.clone())?;
    let redis_pool = Pool::builder().build(manager).await?;

    tracing::info!("Connecting to gRPC backends...");
    let auth_grpc_client =
        grpc_clients::auth_client::AuthGrpcClient::connect(config.auth_service_grpc_url.clone())
            .await
            .expect("Failed to connect to Auth Service");

    let stream_meta_grpc_client = grpc_clients::stream_meta_client::StreamMetaGrpcClient::connect(
        config.stream_meta_grpc_url.clone(),
    )
    .await
    .expect("Failed to connect to Stream Meta Service");

    let state = Arc::new(AppState {
        config: config.clone(),
        redis_pool,
        auth_grpc_client,
        stream_meta_grpc_client,
    });

    let app = Router::new()
        .nest("/v1/auth", api::routes::auth::router())
        .nest("/v1/stream", api::routes::stream::router())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::rate_limiter::rate_limit_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Gateway listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("Received shutdown signal, starting graceful shutdown...");
}
