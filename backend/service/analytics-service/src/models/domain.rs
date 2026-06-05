use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, clickhouse::Row)]
pub struct PingRow {
    pub stream_id: String,
    pub client_ip: String,
    pub timestamp: i64,
}
