use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum IngestError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Authentication Failed: {0}")]
    AuthFailed(String),

    #[error("FFmpeg Process Error: {0}")]
    TranscoderError(String),

    #[error("Client Disconnected")]
    CliendDisconnected,

    #[error("Internal Error: {0}")]
    Internal(#[from] anyhow::Error),
}
