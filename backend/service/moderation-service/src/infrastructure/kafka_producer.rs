use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self { producer })
    }

    pub async fn publish_user_banned(&self, user_id: &str, channel_id: &str, reason: &str) -> anyhow::Result<()> {
        let payload = serde_json::json!({
            "event": "user.banned",
            "user_id": user_id,
            "channel_id": channel_id,
            "reason": reason,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let payload_str = payload.to_string();
        let record = FutureRecord::to("moderation-events")
            .payload(&payload_str)
            .key(user_id);

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => Ok(()),
            Err((e, _)) => Err(anyhow::anyhow!("Failed to send to Kafka: {}", e)),
        }
    }
}
