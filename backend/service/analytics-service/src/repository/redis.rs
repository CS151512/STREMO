use redis::AsyncCommands;
use redis::Client;

#[derive(Clone)]
pub struct RedisRepo {
    client: Client,
}

impl RedisRepo {
    pub fn new(url: &str) -> anyhow::Result<Self> {
        let client = Client::open(url)?;
        Ok(Self { client })
    }

    pub async fn increment_ccv(&self, stream_id: &str) -> anyhow::Result<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let key = format!("ccv:{}", stream_id);
        let _: () = redis::pipe()
            .incr(&key, 1)
            .expire(&key, 30)
            .query_async(&mut con)
            .await?;

        Ok(())
    }

    pub async fn get_current_ccv(&self, stream_id: &str) -> anyhow::Result<i32> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let key = format!("ccv:{}", stream_id);
        let ccv: Option<i32> = con.get(&key).await?;
        Ok(ccv.unwrap_or(0))
    }
}
