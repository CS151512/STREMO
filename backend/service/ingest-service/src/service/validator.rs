use crate::errors::custom::IngestError;

pub struct StreamValidator;

impl StreamValidator {
    pub fn validator_key_format(key: &str) -> Result<(), IngestError> {
        if key.is_empty() {
            return Err(IngestError::Validation(
                "Stream key cannot be empty".to_string(),
            ));
        }

        if key.len() < 32 {
            return Err(IngestError::Validation(
                "Invalid stream key prefix".to_string(),
            ));
        }

        if !key.starts_with("live_") {
            return Err(IngestError::Validation(
                "Stream key must start with 'live_'".to_string(),
            ));
        }

        if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(IngestError::Validation(
                "Stream key must only contain alphanumeric characters or underscores".to_string(),
            ));
        }

        Ok(())
    }
}
