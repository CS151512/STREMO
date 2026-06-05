use crate::infrastructure::{kafka_producer::KafkaProducer, redis_publisher::RedisPublisher};
use crate::repository::postgres::PostgresAuditRepo;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct ModeratorManager {
    audit_repo: Arc<PostgresAuditRepo>,
    kafka: Arc<KafkaProducer>,
    redis: Arc<RedisPublisher>,
    http_client: reqwest::Client,
    ml_service_url: String,
}

#[derive(Serialize)]
struct MlInferenceRequest<'a> {
    text: &'a str,
    user_id: &'a str,
}

#[derive(Deserialize)]
pub struct SpamResult {
    pub is_spam: bool,
    pub confidence: f32,
    pub reason: String,
}

impl ModeratorManager {
    pub fn new(
        audit_repo: PostgresAuditRepo,
        kafka: KafkaProducer,
        redis: RedisPublisher,
    ) -> Self {
        let ml_service_url = std::env::var("ML_SERVICE_URL")
            .unwrap_or_else(|_| "http://ml-spam-filter:8000".to_string());

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_millis(500))
            .build()
            .unwrap();

        Self {
            audit_repo: Arc::new(audit_repo),
            kafka: Arc::new(kafka),
            redis: Arc::new(redis),
            http_client,
            ml_service_url,
        }
    }

    pub async fn check_spam(&self, text: &str, user_id: &str, _channel_id: &str) -> anyhow::Result<SpamResult> {
        let url = format!("{}/v1/predict", self.ml_service_url);

        let request_payload = MlInferenceRequest {
            text,
            user_id,
        };

        let response = self.http_client
            .post(&url)
            .json(&request_payload)
            .send()
            .await?;

        if !response.status().is_success() {
            tracing::error!("ML Service returned HTTP {}", response.status());
            return Err(anyhow::anyhow!("ML Service HTTP Error"));
        }

        let result = response.json::<SpamResult>().await?;
        Ok(result)
    }

    pub async fn ban_user(
        &self,
        user_id: &str,
        channel_id: &str,
        reason: &str,
        moderator_id: &str,
    ) -> anyhow::Result<()> {
        self.audit_repo.log_ban(user_id, channel_id, reason, moderator_id).await?;
        self.redis.publish_ban_event(user_id, channel_id).await?;
        self.kafka.publish_user_banned(user_id, channel_id, reason).await?;

        Ok(())
    }
}
