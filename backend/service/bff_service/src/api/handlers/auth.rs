use axum::{extract::State, Json};
use std::sync::Arc;

use crate::{
    models::{
        requests::{LoginRequest, RegisterRequest},
        responses::{LoginResponse, RegisterResponse},
    },
    utils::errors::AppError,
    utils::state::AppState,
};

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::BadRequest(
            "Email and password are required".to_string(),
        ));
    }

    let mut auth_client = state.auth_grpc_client.clone();

    let grpc_res = auth_client.login(payload.email, payload.password).await?;

    Ok(Json(LoginResponse {
        access_token: grpc_res.access_token,
        refresh_token: grpc_res.refresh_token,
        expires_in: grpc_res.expires_in,
    }))
}

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    if payload.username.is_empty() {
        return Err(AppError::BadRequest("Username is required".to_string()));
    }

    let mut auth_client = state.auth_grpc_client.clone();
    let grpc_res = auth_client
        .register(payload.username, payload.email, payload.password)
        .await?;

    Ok(Json(RegisterResponse {
        id: grpc_res.id,
        message: grpc_res.message,
    }))
}
