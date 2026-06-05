use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsSummaryDto {
    pub stream_id: String,
    pub current_ccv: i32,
    pub peak_ccv: u32,
    pub total_unique_views: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetSummaryQuery {
    pub stream_id: String,
}
