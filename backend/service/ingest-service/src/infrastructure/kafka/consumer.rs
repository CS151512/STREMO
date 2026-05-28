use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use serde::Deserialize;
use tokio_stream::StreamExt;

#[derive(Debug, Deserialize)]
pub struct IngestCommand {
    pub action: String,
    pub stream_id: String,
}

pub async fn start_command_listener(brokers: &str, group_id: &str) -> anyhow::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "latest")
        .create()?;

    consumer.subscribe(&["ingest-commands"])?;
    tracing::info!("Kafka command listener started on group: {}", group_id);

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    let payload_str = String::from_utf8_lossy(payload);
                    tracing::debug!("Received command via Kafka: {}", payload_str);

                    if let Ok(command) = serde_json::from_str::<IngestCommand>(&payload_str) {
                        match command.action.as_str() {
                            "disconnect" | "ban" => {
                                tracing::warn!(
                                    "Executing admin command: {} for stream {}",
                                    command.action,
                                    command.stream_id
                                );

                                //TODO: пробросить сюда Arc<StreamManager>!!! ИИИИ
                                // вызвать manager.stop_stream(&command.stream_id).await;
                            }
                            _ => tracing::debug!("Unknown command action: {}", command.action),
                        }
                    }
                }
            }
            Err(e) => tracing::error!("Kafka error: {}", e),
        }
    }

    Ok(())
}
