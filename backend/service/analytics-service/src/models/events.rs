use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnalyticsEvent {
    StreamStarted {
        stream_id: Uuid,
        timestamp: i64,
    },
    StreamEnded {
        stream_id: Uuid,
        timestamp: i64,
    },
    CcvMilestoneReached {
        stream_id: Uuid,
        ccv: u64,
        timestamp: i64,
    },
}
