use crate::{api::handlers::auth, middleware::rate_limiter, AppState};
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(auth::login_handler))
        .route("/register", post(auth::register_handler))
        .route_layer(axum::middleware::from_fn_with_state(
            //pupupupuppupupu
        ))
}
