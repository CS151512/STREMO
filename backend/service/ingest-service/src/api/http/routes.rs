use crate::infrastructure::metrics::prometheus::metrics_handler;
use axum::{routing::get, Router};

pub async fn start_http_srv(port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/healthz", get(|| async { "OK :)" }))
        .route("/metrics", get(metrics_handler));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("HTTP server listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
