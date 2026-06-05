use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsEvent {
    #[serde(rename = "ccv_update")]
    CcvUpdate { stream_id: String, current_ccv: i32 },
    #[serde(rename = "summary_update")]
    SummaryUpdate {
        stream_id: String,
        current_ccv: i32,
        peak_ccv: u32,
        total_unique_views: u64,
    },
}
