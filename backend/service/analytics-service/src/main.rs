mod api;
mod models;
mod repository;
mod service;

use api::grpc_server::{pb::analytics_service_server::AnalyticsServiceServer, GrpcServerImpl};
use api::http::routes::create_router;
use api::ws::hub::WsHub;
use repository::{clickhouse::ClickHouseRepo, db::PgRepo, redis::RedisRepo};
use service::{aggregator::AnalyticsAggregator, manager::AnalyticsManager};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::transport::Server as GrpcServer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Analytics Service...");

    let clickhouse_url =
        std::env::var("CLICKHOUSE_URL").unwrap_or_else(|_| "http://localhost:8123".to_string());
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let postgres_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/stremo_analytics".to_string()
    });

    let ch_repo = ClickHouseRepo::new(&clickhouse_url);
    let redis_repo = RedisRepo::new(&redis_url)?;

    let pg_repo = PgRepo::new(&postgres_url).await.ok();
    if pg_repo.is_none() {
        tracing::warn!("Failed to connect to PostgreSQL. Running without metadata sync.");
    }

    let aggregator = AnalyticsAggregator::new(ch_repo, redis_repo);
    let ws_hub = WsHub::new();
    let manager = AnalyticsManager::new(aggregator.clone(), pg_repo, ws_hub);

    let grpc_addr = "[::]:50051".parse()?;
    let grpc_service = GrpcServerImpl::new(aggregator);
    let grpc_server = GrpcServer::builder()
        .add_service(AnalyticsServiceServer::new(grpc_service))
        .serve(grpc_addr);

    let http_app = create_router(manager)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let http_addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(http_addr).await?;
    let http_server = axum::serve(listener, http_app);

    tracing::info!("gRPC Server listening on {}", grpc_addr);
    tracing::info!("HTTP/WS Server listening on {}", http_addr);

    tokio::select! {
        res = grpc_server => {
            tracing::error!("gRPC server exited: {:?}", res);
        }
        res = http_server => {
            tracing::error!("HTTP server exited: {:?}", res);
        }
    }

    Ok(())
}
