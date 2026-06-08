use crate::infrastructure::grpc_client::ModerationClient;
use crate::models::domain::ChatMessage;
use crate::repository::{postgres::PostgresBatcher, redis_pubsub::RedisPubSub};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

#[derive(Clone)]
pub struct ChatManager {
    redis: Arc<RedisPubSub>,
    db_batcher: Arc<PostgresBatcher>,
    mod_client: Arc<ModerationClient>,
    local_channels: Arc<Mutex<HashMap<String, broadcast::Sender<ChatMessage>>>>,
}

impl ChatManager {
    pub fn new(redis: RedisPubSub, db_batcher: PostgresBatcher, mod_client: ModerationClient) -> Self {
        Self {
            redis: Arc::new(redis),
            db_batcher: Arc::new(db_batcher),
            mod_client: Arc::new(mod_client),
            local_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn handle_incoming_message(&self, mut msg: ChatMessage) -> anyhow::Result<()> {
        if self.mod_client.is_spam(&msg.text, &msg.user_id, &msg.channel_id).await? {
            tracing::warn!("Spam detected from user {}, dropping message", msg.user_id);
            return Ok(());
        }

        msg.timestamp = chrono::Utc::now().timestamp_millis();

        self.redis.publish(&msg.channel_id, &msg).await?;
        self.db_batcher.add_message(msg).await;

        Ok(())
    }

    pub async fn get_history(&self, channel_id: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<ChatMessage>> {
        self.db_batcher.get_history(channel_id, limit, offset).await
    }

    pub async fn subscribe_local(&self, channel_id: &str) -> broadcast::Receiver<ChatMessage> {
        let mut channels = self.local_channels.lock().await;

        if let Some(tx) = channels.get(channel_id) {
            return tx.subscribe();
        }

        let (tx, rx) = broadcast::channel(100);
        channels.insert(channel_id.to_string(), tx.clone());

        let redis_clone = self.redis.clone();
        let cid = channel_id.to_string();

        tokio::spawn(async move {
            if let Err(e) = redis_clone.subscribe_to_channel(&cid, tx).await {
                tracing::error!("Failed to subscribe to redis channel {}: {}", cid, e);
            }
        });

        rx
    }
}
