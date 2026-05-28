use pb::auth_service_client::AuthServiceClient;
use pb::ValidateStreamKeyRequest;
use std::time::Duration;
use tonic::transport::{Channel, Endpoint};

pub mod pb {
    tonic::include_proto!("stremo.auth.v1");
}

#[derive(Clone)]
pub struct AuthGrpcClient {
    client: AuthServiceClient<Channel>,
}

impl AuthGrpcClient {
    pub async fn connect(url: String) -> anyhow::Result<Self> {
        let endpoint = Endpoint::from_shared(url)?
            .timeout(Duration::from_secs(5))
            .concurrency_limit(256)
            .tcp_keepalive(Some(Duration::from_secs(15)));

        let channel = endpoint.connect().await?;
        let client = AuthServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn verify_stream_key(&self, stream_key: &str) -> anyhow::Result<bool> {
        let mut client = self.client.clone();

        let request = tonic::Request::new(ValidateStreamKeyRequest {
            stream_key: stream_key.to_string(),
        });

        match client.validate_stream_key(request).await {
            Ok(response) => {
                let inner = response.into_inner();
                tracing::debug!("Validated stream key for channel_id: {}", inner.channel_id);
                Ok(!inner.channel_id.is_empty())
            }
            Err(status) => {
                tracing::warn!("gRPC auth verification failed: {}", status.message());
                Ok(false)
            }
        }
    }
}
