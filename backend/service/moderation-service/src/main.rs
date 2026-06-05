mod api;
mod infrastructure;
mod repository;
mod service;

use api::grpc_server::{GrpcServerImpl, pb::moderation_service_server::ModerationServiceServer};
use infrastructure::{kafka_producer::KafkaProducer, redis_publisher::RedisPublisher};
use repository::postgres::PostgresAuditRepo;
use service::moderator::ModeratorManager;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Moderation Service...");

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let kafka_brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost/stremo".to_string());

    let redis_publisher = RedisPublisher::new(&redis_url)?;
    let kafka_producer = KafkaProducer::new(&kafka_brokers)?;
    let audit_repo = PostgresAuditRepo::new(&db_url).await?;
    let manager = ModeratorManager::new(audit_repo, kafka_producer, redis_publisher);
    let addr = "[::]:50051".parse()?;
    let grpc_service = GrpcServerImpl::new(manager);

    tracing::info!("Moderation gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(ModerationServiceServer::new(grpc_service))
        .serve(addr)
        .await?;

    Ok(())
}
