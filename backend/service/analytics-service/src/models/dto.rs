use serde::{Deserialize, Serialize};
use uuid::Uuid;

//GET /api/v1/summary/:stream_id
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamSummaryResponse {
    pub stream_id: Uuid,
    pub currnet_viewers: u64,
    pub peak_viewers: u64,
    pub is_live: bool,
}

impl From<crate::models::domain::StreamSummary> for StreamSummaryResponse {
    fn from(domain: crate::models::domain::StreamSummary) -> Self {
        Self {
            stream_id: domain.stream_id,
            currnet_viewers: domain.current_ccv,
            peak_viewers: domain.peak_ccv,
            is_live: domain.current_ccv > 0,
        }
    }
}
