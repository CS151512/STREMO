use super::aggregator::AnalyticsAggregator;
use super::validator::validate_stream_id;
use crate::api::ws::hub::WsHub;
use crate::models::dto::AnalyticsSummaryDto;
use crate::models::events::WsEvent;
use crate::repository::db::PgRepo;
use std::sync::Arc;
use tokio::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct AnalyticsManager {
    pub aggregator: AnalyticsAggregator,
    pub pg_repo: Option<PgRepo>,
    pub ws_hub: WsHub,
}

impl AnalyticsManager {
    pub fn new(aggregator: AnalyticsAggregator, pg_repo: Option<PgRepo>, ws_hub: WsHub) -> Self {
        let manager = Self {
            aggregator,
            pg_repo,
            ws_hub,
        };

        manager.start_ccv_broadcaster();
        manager
    }

    pub async fn get_summary(&self, stream_id: &str) -> anyhow::Result<AnalyticsSummaryDto> {
        validate_stream_id(stream_id)?;

        let summary = self.aggregator.get_stream_summary(stream_id).await?;

        Ok(AnalyticsSummaryDto {
            stream_id: stream_id.to_string(),
            current_ccv: summary.current_ccv,
            peak_ccv: summary.peak_ccv,
            total_unique_views: summary.total_unique_views,
        })
    }

    fn start_ccv_broadcaster(&self) {
        tracing::debug!("Started CCV broadcaster (stub)");
    }

    pub async fn notify_ccv_update(&self, stream_id: &str, current_ccv: i32) {
        self.ws_hub
            .broadcast(
                &stream_id.to_string(),
                WsEvent::CcvUpdate {
                    stream_id: stream_id.to_string(),
                    current_ccv,
                },
            )
            .await;
    }
}
