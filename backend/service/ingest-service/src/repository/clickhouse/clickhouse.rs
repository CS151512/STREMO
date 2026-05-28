use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};

pub struct ClickhouseRepository {
    client: Client,
}

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct CCVMetrics {
    pub stream_id: String,
    pub view_cnt: u32,
    pub timestamp: u64,
}

impl ClickhouseRepository {
    pub fn new(url: &str) -> Self {
        let client = Client::default()
            .with_url(url)
            .with_database("stremo_analytics");
        Self { client }
    }

    pub async fn insert_ccv_batch(&self, metrics: &[CCVMetrics]) -> anyhow::Result<()> {
        let mut insert = self.client.insert("ccv_metrics");
        for metric in metrics {
            insert.write(metric).await?;
        }
        insert.end().await?;
        Ok(())
    }
}
