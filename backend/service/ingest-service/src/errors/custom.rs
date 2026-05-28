use thiserror::Error;

#[derive(Error, Debug)]
pub enum IngestError {
    #[error("Stream validation failed: {0}")]
    Validation(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Infrastructure error: {0}")]
    Infra(#[from] anyhow::Error),
}
