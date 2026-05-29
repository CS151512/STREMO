use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

use super::queries;

pub struct PostgresRepo {
    pool: Pool,
}

impl PostgresRepo {
    pub async fn new(dns: &str) -> anyhow::Result<Self> {
        let mut cfg = Config::new();
        cfg.url = Some(dns.to_string());
        cfg.manager = Some(deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        });

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
        Ok(Self { pool })
    }

    pub async fn get_stream(
        &self,
        stream_id: &uuid::Uuid,
    ) -> anyhow::Result<Option<crate::models::domain::Stream>> {
        let client = self.pool.get().await?;

        let row_opt = client
            .query_opt(queries::GET_STREAM_QUERY, &[stream_id])
            .await?;

        if let Some(row) = row_opt {
            let id: uuid::Uuid = row.get("id");
            let channel_id: String = row.get("channel_id");
            let status_str: String = row.get("status");

            let status = match status_str.as_str() {
                "active" => crate::models::domain::StreamStatus::Active,
                "finished" => crate::models::domain::StreamStatus::Finished,
                _ => crate::models::domain::StreamStatus::Pending,
            };

            Ok(Some(crate::models::domain::Stream {
                stream_id: id,
                channel_id,
                status,
            }))
        } else {
            Ok(None)
        }
    }
}
