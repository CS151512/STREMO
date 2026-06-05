use crate::models::domain::PingRow;
use clickhouse::Client;

#[derive(Clone)]
pub struct ClickHouseRepo {
    client: Client,
}

impl ClickHouseRepo {
    pub fn new(url: &str) -> Self {
        let client = Client::default()
            .with_url(url)
            .with_database("stremo_analytics");
        Self { client }
    }

    pub async fn bulk_insert_pings(&self, pings: &[PingRow]) -> anyhow::Result<()> {
        if pings.is_empty() {
            return Ok(());
        }

        let mut insert = self.client.insert("stream_pings")?;
        for ping in pings {
            insert.write(ping).await?;
        }
        insert.end().await?;

        tracing::debug!(
            "Successfully bulk inserted {} pings to ClickHouse",
            pings.len()
        );
        Ok(())
    }

    pub async fn get_total_unique_views(&self, stream_id: &str) -> anyhow::Result<u64> {
        let count = self
            .client
            .query("SELECT count(distinct client_ip) FROM stream_pings WHERE stream_id = ?")
            .bind(stream_id)
            .fetch_one::<u64>()
            .await
            .unwrap_or(0);

        Ok(count)
    }

    pub async fn get_peak_ccv(&self, stream_id: &str) -> anyhow::Result<u32> {
        let query = "
            SELECT toUInt32(max(ccv)) FROM (
                SELECT toStartOfMinute(toDateTime(timestamp)) as t, count(distinct client_ip) as ccv
                FROM stream_pings
                WHERE stream_id = ?
                GROUP BY t
            )
        ";

        let peak = self
            .client
            .query(query)
            .bind(stream_id)
            .fetch_optional::<u32>()
            .await
            .unwrap_or(Some(0))
            .unwrap_or(0);

        Ok(peak)
    }
}
