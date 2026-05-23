use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use tracing_subscriber::fmt::format;

use crate::{
    models::requests,
    utils::{errors::AppError, state::AppState},
};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|val| val.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|val| val.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "unknown_ip".to_string());

    let redis_key = format!("rate_limit:ip: {}", ip);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let window_start = now - 60_000;
    let request_id = Uuid::new_v4().to_string();

    let mut redis_conn = state.redis_pool.get().await.map_err(|e| {
        tracing::error!("Redis pool error: {}", e);
        AppError::InternalError(anyhow::anyhow!("DB Connection Error"))
    })?;

    let (_, _, count, _): ((), (), i32, ()) = redis::pipe()
        .atomic()
        .zrembyscore(&redis_key, 0, window_start as i64)
        .zadd(&redis_key, &request_id, now as i64)
        .zcard(&redis_key)
        .expire(&redis_key, 60)
        .query_async(&mut *redis_conn)
        .await
        .map_err(|e| {
            tracing::error!("Redis pipeline error: {}", e);
            AppError::InternalError(anyhow::anyhow!("Redis Error"))
        })?;

    if count > 100 {
        tracing::warn!(
            "Rate limit exceeded (Sliding Window) for IP: {} (count: {})",
            ip,
            count
        );
        return Err(AppError::BadRequest("Too Many Requests".to_string()));
    }

    Ok(next.run(req).await)
}
