use crate::models::domain::ChatMessage;
use futures::StreamExt;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct RedisPubSub {
    client: redis::Client,
}

impl RedisPubSub {
    pub fn new(url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(url)?;
        Ok(Self { client })
    }

    pub async fn publish(&self, channel_id: &str, message: &ChatMessage) -> anyhow::Result<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let redis_channel = format!("chat:{}", channel_id);
        let payload = serde_json::to_string(message)?;

        let _: () = con.publish(redis_channel, payload).await?;
        Ok(())
    }

    pub async fn subscribe_to_channel(&self, channel_id: &str, tx: broadcast::Sender<ChatMessage>) -> anyhow::Result<()> {
        let redis_channel = format!("chat:{}", channel_id);
        let mut pubsub = self.client.get_async_pubsub().await?;

        pubsub.subscribe(&redis_channel).await?;
        tracing::debug!("Subscribed to Redis channel: {}", redis_channel);

        let mut stream = pubsub.into_on_message();

        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&payload) {
                        if tx.send(chat_msg).is_err() {
                            tracing::debug!("No local subscribers left for {}, closing redis sub", redis_channel);
                            break;
                        }
                    }
                }
            }
        });

        Ok(())
    }
}
