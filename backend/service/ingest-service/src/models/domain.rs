use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Stream {
    pub stream_id: Uuid,
    pub channel_id: String,
    pub status: StreamStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StreamStatus {
    Pending,
    Active,
    Finished,
}
