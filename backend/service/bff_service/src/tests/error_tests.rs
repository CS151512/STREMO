use crate::utils::errors::AppError;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[test]
fn test_invalid_credentials_mapping() {
    let err = AppError::InvalidCredentials;
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_bad_request_mapping() {
    let err = AppError::BadRequest("Missing field".to_string());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_grpc_error_mapping() {
    let err = AppError::GrpcError("Connection refused".to_string());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
}

#[test]
fn test_internal_error_mapping() {
    let err = AppError::InternalError(anyhow::anyhow!("DB exploded"));
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_tonic_status_conversion() {
    let status = tonic::Status::new(tonic::Code::Unavailable, "Server is down");
    let app_error: AppError = status.into();

    match app_error {
        AppError::GrpcError(msg) => assert_eq!(msg, "Server is down"),
        _ => panic!("Expected GrpcError"),
    }
}
