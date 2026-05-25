use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum IngestEvent {
    StreamStarted {
        stream_id: String,
        channel_id: String,
    },
    StreamEnded {
        stream_id: String,
    },
}
