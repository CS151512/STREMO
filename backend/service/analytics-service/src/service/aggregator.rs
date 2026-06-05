use crate::models::domain::PingRow;
use crate::repository::{clickhouse::ClickHouseRepo, redis::RedisRepo};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AnalyticsAggregator {
    buffer: Arc<Mutex<Vec<PingRow>>>,
    ch_repo: Arc<ClickHouseRepo>,
    redis_repo: Arc<RedisRepo>,
}

pub struct StreamSummary {
    pub current_ccv: i32,
    pub peak_ccv: u32,
    pub total_unique_views: u64,
}

impl AnalyticsAggregator {
    pub fn new(ch_repo: ClickHouseRepo, redis_repo: RedisRepo) -> Self {
        let aggregator = Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            ch_repo: Arc::new(ch_repo),
            redis_repo: Arc::new(redis_repo),
        };

        aggregator.start_flush_worker();

        aggregator
    }

    pub async fn process_ping(&self, ping: PingRow) -> anyhow::Result<()> {
        self.redis_repo.increment_ccv(&ping.stream_id).await?;

        let mut buf = self.buffer.lock().await;
        buf.push(ping);

        Ok(())
    }
    pub async fn get_stream_summary(&self, stream_id: &str) -> anyhow::Result<StreamSummary> {
        let current_ccv_future = self.redis_repo.get_current_ccv(stream_id);
        let peak_ccv_future = self.ch_repo.get_peak_ccv(stream_id);
        let total_views_future = self.ch_repo.get_total_unique_views(stream_id);

        let (current_ccv_res, peak_ccv_res, total_views_res) =
            tokio::join!(current_ccv_future, peak_ccv_future, total_views_future);
        let current_ccv = current_ccv_res.unwrap_or(0);

        let ch_peak = peak_ccv_res.unwrap_or(0);
        let peak_ccv = std::cmp::max(ch_peak, current_ccv as u32);

        let total_unique_views = total_views_res.unwrap_or(0);

        Ok(StreamSummary {
            current_ccv,
            peak_ccv,
            total_unique_views,
        })
    }

    fn start_flush_worker(&self) {
        let buffer_clone = self.buffer.clone();
        let ch_repo_clone = self.ch_repo.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;

                let pings_to_insert = {
                    let mut buf = buffer_clone.lock().await;
                    if buf.is_empty() {
                        continue;
                    }
                    std::mem::take(&mut *buf)
                };

                if let Err(e) = ch_repo_clone.bulk_insert_pings(&pings_to_insert).await {
                    tracing::error!("Failed to flush pings to ClickHouse: {}", e);
                    // реализовать retry-логику когда-нибудь ......
                }
            }
        });
    }
}
