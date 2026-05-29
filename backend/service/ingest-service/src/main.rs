mod api;
mod config;
mod errors;
mod infrastructure;
mod models;
mod repository;
mod service;
mod utils;

use infrastructure::{
    ffmpeg::FFmpegRunner, grpc_client::AuthGrpcClient, kafka::consumer,
    kafka::producer::KafkaProducer, logger, metrics::prometheus,
};
use service::manager::StreamManager;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    tracing::info!("Starting ingest service");

    let config = config::Config::from_env();

    prometheus::registry_metrics();

    if let Err(e) = tokio::fs::create_dir_all(&config.hls_output_dir).await {
        tracing::warn!("Failed to create HLS output directory: {}", e);
    }

    tracing::info!(
        "Connecting to Auth Service at {}",
        config.auth_service_grpc_url
    );
    let auth_client = AuthGrpcClient::connect(config.auth_service_grpc_url.clone()).await?;

    let ffmpeg_runner = Arc::new(FFmpegRunner::new(
        config.ffmpeg_path.clone(),
        config.hls_output_dir.clone(),
    ));

    let kafka_producer = KafkaProducer::new(&config.kafka_brokers)?;

    let postgres_repo = Arc::new(crate::repository::postgres::PostgresRepo::new(
        "host=localhost user=postgres dbname=stremo",
    )?);
    let redis_cache =
        Arc::new(crate::repository::redis::cache::RedisCache::new("redis://127.0.0.1/").await?);

    let stream_manager = StreamManager::new(
        auth_client,
        ffmpeg_runner,
        kafka_producer,
        postgres_repo,
        redis_cache,
    );

    let http_task = tokio::spawn(async move {
        let _ = api::http::routes::start_http_server(9090).await;
    });

    let kafka_brokers_clone = config.kafka_brokers.clone();
    let kafka_task = tokio::spawn(async move {
        if let Err(e) = api::tcp_rtmp::start_server(port, stream_manager).await {
            tracing::error!("TCP Server crashed: {}", e);
        }
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutting down");

        }
        _ = server_task => {}
        _ = http_task => {}
        _ = kafka_task => {}
    }

    Ok(())
}
