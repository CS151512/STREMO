use crate::errors::custom::IngestError;
use crate::infrastructure::{
    ffmpeg::FFmpegRunner, grpc_client::AuthGrpcClient, kafka::producer::KafkaProducer,
};
use crate::repository::{postgres::PostgresRepo, redis::cache::RedisCache};
use crate::service::{logic::StreamLogic, validator::StreamValidator};
use std::sync::Arc;
use uuid::Uuid;

pub struct StreamManager {
    auth_client: Arc<AuthGrpcClient>,
    ffmpeg_runner: Arc<FFmpegRunner>,
    kafka_producer: Arc<KafkaProducer>,
    postgres_repo: Arc<PostgresRepo>,
    redis_cache: Arc<RedisCache>,
}

impl StreamManager {
    pub fn new(
        auth_client: AuthGrpcClient,
        ffmpeg_runner: Arc<FFmpegRunner>,
        kafka_producer: KafkaProducer,
        postgres_repo: Arc<PostgresRepo>,
        redis_cache: Arc<RedisCache>,
    ) -> Self {
        Self {
            auth_client: Arc::new(auth_client),
            ffmpeg_runner,
            kafka_producer: Arc::new(kafka_producer),
            postgres_repo,
            redis_cache,
        }
    }

    pub async fn validate_and_start_stream(
        &self,
        stream_key: &str,
        client_ip: &str,
    ) -> Result<Uuid, IngestError> {
        // 1. Rate Limiting через Redis (Защита от брутфорса stream key)
        if self
            .redis_cache
            .is_rate_limited(client_ip, 10, 60)
            .await
            .map_err(|e| IngestError::Infra(e))?
        {
            return Err(IngestError::Validation(
                "Too many connection attempts. Please try again later.".to_string(),
            ));
        }

        StreamValidator::validator_key_format(stream_key)?;

        let is_valid = self
            .auth_client
            .verify_stream_key(stream_key)
            .await
            .map_err(|e| IngestError::Auth(e.to_string()))?;

        if !is_valid {
            return Err(IngestError::Auth(
                "Invalid stream key provided by Auth Service".to_string(),
            ));
        }

        //REFACTOR: Auth Service должен возвращать настоящий UUID сессии стрима!!!!!!!! ниже переделать
        let stream_id = self
            .auth_client
            .get_stream_id(stream_key)
            .await
            .map_err(|e| IngestError::Auth(e.to_string()))?;

        if let Ok(Some(stream_record)) = self.postgres_repo.get_stream(&stream_id).await {
            StreamLogic::can_start_stream(&stream_record)?;
        } else {
            return Err(IngestError::Validation(
                "Stream not found in the database".to_string(),
            ));
        }

        let stream_id_str = stream_id.to_string();
        self.ffmpeg_runner
            .start_transcoder(&stream_id_str)
            .await
            .map_err(|e| IngestError::Infra(e))?;

        let event_payload = serde_json::json!({
            "event": "stream_started",
            "stream_id": stream_id_str,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        self.kafka_producer
            .send_event("stream-events", &stream_id_str, &event_payload.to_string())
            .await
            .map_err(|e| IngestError::Infra(e))?;

        Ok(stream_id)
    }

    pub async fn stop_stream(&self, stream_id: &Uuid) -> Result<(), IngestError> {
        let stream_id_str = stream_id.to_string();

        self.ffmpeg_runner
            .stop_transcoder(&stream_id_str)
            .await
            .map_err(|e| IngestError::Infra(e))?;

        let event_payload = serde_json::json!({
            "event": "stream_ended",
            "stream_id": stream_id_str,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        self.kafka_producer
            .send_event("stream-events", &stream_id_str, &event_payload.to_string())
            .await
            .map_err(|e| IngestError::Infra(e))?;

        self.postgres_repo
            .update_stream_status(stream_id, StreamStatus::Finished)
            .await?;

        Ok(())
    }
}
