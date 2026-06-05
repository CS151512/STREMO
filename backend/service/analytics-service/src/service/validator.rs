use validator::Validate;

#[derive(Debug, Validate)]
pub struct StreamIdValidation {
    #[validate(length(min = 1, message = "Stream ID cannot be empty"))]
    pub stream_id: String,
}

pub fn validate_stream_id(stream_id: &str) -> Result<(), validator::ValidationErrors> {
    let check = StreamIdValidation {
        stream_id: stream_id.to_string(),
    };
    check.validate()
}
