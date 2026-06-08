use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    response::IntoResponse,
    http::StatusCode,
};
use crate::service::manager::ChatManager;
use crate::models::domain::ChatMessage;
use std::sync::Arc;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

#[derive(Deserialize)]
pub struct WsQuery {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: usize,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(channel_id): Path<String>,
    Query(query): Query<WsQuery>,
    State(manager): State<Arc<ChatManager>>,
) -> impl IntoResponse {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let token_data = match decode::<Claims>(
        &query.token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Invalid WS token: {}", e);
            return (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response();
        }
    };

    let user_id = token_data.claims.sub;
    let username = token_data.claims.username;

    ws.on_upgrade(move |socket| handle_socket(socket, channel_id, manager, user_id, username))
}

async fn handle_socket(
    socket: WebSocket,
    channel_id: String,
    manager: Arc<ChatManager>,
    user_id: String,
    username: String,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = manager.subscribe_local(&channel_id).await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(payload) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(payload)).await.is_err() {
                    break;
                }
            }
        }
    });

    let manager_clone = manager.clone();
    let cid = channel_id.clone();
    let uid = user_id.clone();
    let uname = username.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(mut chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                chat_msg.channel_id = cid.clone();
                chat_msg.user_id = uid.clone();
                chat_msg.username = uname.clone();
                if uuid::Uuid::parse_str(&chat_msg.id).is_err() {
                    chat_msg.id = uuid::Uuid::new_v4().to_string();
                }

                if let Err(e) = manager_clone.handle_incoming_message(chat_msg).await {
                    tracing::error!("Error handling message from {}: {}", uid, e);
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
