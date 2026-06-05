use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisPublisher {
    client: redis::Client,
}

impl RedisPublisher {
    pub fn new(url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(url)?;
        Ok(Self { client })
    }

    pub async fn publish_ban_event(&self, user_id: &str, channel_id: &str) -> anyhow::Result<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;

        let payload = serde_json::json!({
            "action": "disconnect",
            "user_id": user_id,
            "channel_id": channel_id,
        });
        let redis_channel = format!("moderation:channel:{}", channel_id);
        let _: () = con.publish(redis_channel, payload.to_string()).await?;

        Ok(())
    }
}
