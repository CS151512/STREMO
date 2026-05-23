use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalError(anyhow::Error),

    #[error("Invalid Credentials")]
    InvalidCredentials,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("gRPC Error: {0}")]
    GrpcError(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, error_message) = match self {
            AppError::InternalError(ref e) => {
                tracing::error!("Internal Server Error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Что-то пошло не так на сервере".to_string(),
                )
            }
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "Неверный email или пароль".to_string(),
            ),
            AppError::Unauthorized(ref msg) => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.to_string())
            }
            AppError::BadRequest(ref msg) => {
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.to_string())
            }
            AppError::GrpcError(ref msg) => {
                tracing::error!("gRPC call failed: {}", msg);
                (
                    StatusCode::BAD_GATEWAY,
                    "UPSTREAM_ERROR",
                    "Внутренний сервис недоступен".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": error_message
            }
        }));

        (status, body).into_response()
    }
}

impl From<tonic::Status> for AppError {
    fn from(status: tonic::Status) -> Self {
        AppError::GrpcError(status.message().to_string())
    }
}
