use pb::moderation_service_client::ModerationServiceClient;
use pb::CheckSpamRequest;
use std::time::Duration;
use tonic::transport::{Channel, Endpoint};

pub mod pb {
    tonic::include_proto!("stremo.moderation.v1");
}

#[derive(Clone)]
pub struct ModerationClient {
    client: ModerationServiceClient<Channel>,
}

impl ModerationClient {
    pub async fn connect(url: String) -> anyhow::Result<Self> {
        let endpoint = Endpoint::from_shared(url)?
            .timeout(Duration::from_millis(500))
            .concurrency_limit(1024);

        let channel = endpoint.connect().await?;
        let client = ModerationServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn is_spam(&self, text: &str, user_id: &str, channel_id: &str) -> anyhow::Result<bool> {
        let mut client = self.client.clone();

        let request = tonic::Request::new(CheckSpamRequest {
            message_text: text.to_string(),
            user_id: user_id.to_string(),
            channel_id: channel_id.to_string(),
        });

        match client.check_spam(request).await {
            Ok(response) => Ok(response.into_inner().is_spam),
            Err(e) => {
                tracing::warn!("Moderation service error: {}, allowing message.", e.message());
                Ok(false)
            }
        }
    }
}
