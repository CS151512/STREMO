use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamKeyValidationReq {
    pub stream_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamKeyValidationRes {
    pub is_valid: bool,
    pub channel_id: Option<String>,
}
