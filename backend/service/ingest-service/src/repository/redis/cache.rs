use redis::aio::MultiplexedConnection;
use redis::Client;

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(url: &str) -> Self {
        let client = Client::open(url)?;
        Self { client }
    }

    pub async fn get_connection(&self) -> anyhow::Result<MultiplexedConnection> {
        let conn = self.client.get_multiplexed_async_connection().await?;
        Ok(conn)
    }

    pub async fn is_rate_limited(
        &self,
        ip: &str,
        limit: u32,
        window_secs: usize,
    ) -> anyhow::Result<bool> {
        let mut conn = self.get_connection().await?;
        let key = format!("rate_limit:{}", ip);

        let count: redis::RedisResult<u32> =
            redis::cmd("INCR").arg(&key).query_async(&mut conn).await?;
        let count = count?;

        if count == 1 {
            let _: () = redis::cmd("EXPIRE")
                .arg(&key)
                .arg(window_secs)
                .query_async(&mut conn)
                .await?;
        }
        Ok(count > limit)
    }
}
