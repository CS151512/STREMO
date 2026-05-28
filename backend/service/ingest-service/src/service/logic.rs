use crate::errors::custom::IngestError;
use crate::models::domain::{Stream, StreamStatus};

pub struct StreamLogic;

impl StreamLogic {
    pub fn can_start_stream(stream: &Stream) -> Result<(), IngestError> {
        match stream.status {
            StreamStatus::Finished => Err(IngestError::Validation(
                "Cannot restart a finished stream session. Please generate a new session."
                    .to_string(),
            )),
            StreamStatus::Active => Err(IngestError::Validation(
                "Stream is already active. Concurrent broadcasting is not allowed.".to_string(),
            )),
            StreamStatus::Pending => Ok(()),
        }
    }

    pub fn generate_hls_output_path(stream_id: &uuid::Uuid, base_dir: &str) -> String {
        let timestamp = chrono::Utc::now().timestamp();
        format!("{}/stream_{}_{}", base_dir, stream_id, timestamp)
    }

    pub fn calculate_duration(
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> i64 {
        let duration = end_time.signed_duration_since(start_time);
        duration.num_seconds()
    }
}
