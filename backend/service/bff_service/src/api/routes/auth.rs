use crate::{api::handlers::auth, utils::state::AppState};
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(auth::login_handler))
        .route("/register", post(auth::register_handler))
}
