use serde::Serialize;
use std::time::Duration;

#[derive(Clone)]
pub struct WebhookClient {
    client: reqwest::Client,
    vod_service_url: String,
}

#[derive(Serialize)]
struct VodReadyPayload<'a> {
    stream_id: &'a str,
    master_playlist_url: &'a str,
}

impl WebhookClient {
    pub fn new(vod_service_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            client,
            vod_service_url,
        }
    }

    pub async fn notify_vod_ready(&self, stream_id: &str, playlist_path: &str) -> anyhow::Result<()> {
        let url = format!("{}/webhook/vod-ready", self.vod_service_url);

        let payload = VodReadyPayload {
            stream_id,
            master_playlist_url: playlist_path,
        };

        tracing::info!("Sending VOD ready webhook for stream: {}", stream_id);

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            tracing::error!("VOD Service rejected webhook: {}", response.status());
            return Err(anyhow::anyhow!("Webhook failed"));
        }

        Ok(())
    }
}
