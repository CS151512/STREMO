use crate::service::manager::StreamManager;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn start_server(port: u16, manager: StreamManager) -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on port {}", port);

    let manager = Arc::new(manager);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        tracing::info!("Accepted connection from {}", addr);
        let manager_clone = manager.clone();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return, // deadin connectrion
                Ok(n) => {
                    let payload = String::from_utf8_lossy(&buf[..n]);
                    let stream_key = payload.trim().to_string();

                    let client_ip = addr.ip().to_string();
                    tracing::info!("Validating stream key from {}", addr);

                    match manager_clone
                        .validate_and_start_stream(&stream_key, &client_ip)
                        .await
                    {
                        Ok(stream_id) => {
                            let _ = socket.write_all(b"OK").await;

                            tracing::info!(
                                "Stream {} authenticated and flowing (ID: {})",
                                stream_key,
                                stream_id
                            );

                            loop {
                                let mut stream_buf = vec![0; 8192];
                                match socket.read(&mut stream_buf).await {
                                    Ok(0) => break,
                                    Ok(_n) => {
                                        crate::infrastructure::metrics::prometheus::INGESTED_BYTES
                                            .inc_by(_n as u64);
                                    }
                                    Err(_) => break,
                                }
                            }

                            tracing::info!("Stream {} disconnected", stream_key);
                            let _ = manager_clone.stop_stream(&stream_id).await;
                        }
                        Err(e) => {
                            tracing::warn!("Invalid stream key from {}: {:?}", addr, e);
                            let _ = socket.write_all(b"ERROR: Invalid Key").await;
                        }
                    }
                }
                Err(e) => tracing::error!("Failed to read from socket: {}", e),
            }
        });
    }
}
