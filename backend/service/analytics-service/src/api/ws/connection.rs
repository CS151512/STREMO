use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use super::hub::WsHub;

pub async fn handle_ws_connection(mut socket: WebSocket, stream_id: String, hub: WsHub) {
    let client_id = Uuid::new_v4();
    let (tx, mut rx) = mpsc::channel(100);

    hub.subscribe(stream_id.clone(), client_id, tx).await;
    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let Ok(msg) = serde_json::to_string(&event) {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    });
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Close(_) = msg {
                break;
            }
        }
    });
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    hub.unsubscribe(&stream_id, &client_id).await;
}
