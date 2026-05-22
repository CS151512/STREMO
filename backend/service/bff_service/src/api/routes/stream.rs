use crate::{api::handlers::stream, AppState};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/live", get(stream::get_live_streams_handler))
}
