use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Base metrics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewerPing {
    pub stream_id: Uuid,
    pub viewer_id: Uuid,
    pub timestamp: i64,
}

// Aggregated stats
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamSummary {
    pub stream_id: Uuid,
    pub current_ccv: u64,
    pub peak_ccv: u64,
    pub started_at: i64,
}
