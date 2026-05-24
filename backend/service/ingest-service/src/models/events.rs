use serd::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamStartedEvent {
    pub channel_id: String,
    pub timestamp: u64,
}

#[derive(Debugm, Serialize, Deserialize)]
pub struct StreamEndedEvent {
    pub channel_id: String,
    pub bytes_transfered: u64,
    pub timestamp: u64,
}
