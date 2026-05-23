use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{decode, DecodingKey, Validation};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{utils::errors::AppError, utils::state::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: usize,
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Token is missing".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Invalid token format".to_string()));
        }

        let token = &auth_header[7..];

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

        let mut redis_conn = state.redis_pool.get().await.map_err(|e| {
            tracing::error!("Redis pool error: {}", e);
            AppError::InternalError(anyhow::anyhow!("DB Connection Error"))
        })?;

        let blacklist_key = format!("blacklist:{}", token_data.claims.sub);
        let is_blacklisted: bool = redis_conn
            .exists(&blacklist_key)
            .await
            .map_err(|_| AppError::InternalError(anyhow::anyhow!("Redis Error")))?;

        if is_blacklisted {
            tracing::warn!(
                "Access attempt with revoked token: {}",
                token_data.claims.sub
            );
            return Err(AppError::Unauthorized(
                "Session revoked. Re-login required.".to_string(),
            ));
        }

        Ok(token_data.claims)
    }
}
