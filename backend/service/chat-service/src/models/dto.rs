use crate::models::domain::ChatMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatHistoryQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ChatHistoryResponse {
    pub messages: Vec<ChatMessage>,
}
