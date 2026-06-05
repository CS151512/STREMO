use crate::models::events::WsEvent;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

type StreamId = String;
type ClientId = uuid::Uuid;

#[derive(Clone)]
pub struct WsHub {
    subscribers: Arc<RwLock<HashMap<StreamId, HashMap<ClientId, mpsc::Sender<WsEvent>>>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe(
        &self,
        stream_id: StreamId,
        client_id: ClientId,
        tx: mpsc::Sender<WsEvent>,
    ) {
        let mut subs = self.subscribers.write().await;
        subs.entry(stream_id).or_default().insert(client_id, tx);
    }

    pub async fn unsubscribe(&self, stream_id: &StreamId, client_id: &ClientId) {
        let mut subs = self.subscribers.write().await;
        if let Some(stream_subs) = subs.get_mut(stream_id) {
            stream_subs.remove(client_id);
            if stream_subs.is_empty() {
                subs.remove(stream_id);
            }
        }
    }

    pub async fn broadcast(&self, stream_id: &StreamId, event: WsEvent) {
        let subs = self.subscribers.read().await;
        if let Some(stream_subs) = subs.get(stream_id) {
            for tx in stream_subs.values() {
                let _ = tx.send(event.clone()).await;
            }
        }
    }
}
