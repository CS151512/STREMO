use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

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

    pub async fn send_event(&self, topic: &str, key: &str, payload: &str) -> anyhow::Result<()> {
        let record = FutureRecord::to(topic).payload(payload).key(key);

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(delivery) => {
                tracing::debug!("Sent event to Kafka: {:?}", delivery);
                Ok(())
            }
            Err((e, _)) => {
                tracing::error!("Failed to send event to Kafka: {}", e);
                Err(anyhow::anyhow!("Kafka error: {}", e))
            }
        }
    }
}
