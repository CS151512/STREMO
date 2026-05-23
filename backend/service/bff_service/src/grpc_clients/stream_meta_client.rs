use super::pb::stream_meta::{
    stream_meta_service_client::StreamMetaServiceClient, GetLiveStreamsRequest,
    GetLiveStreamsResponse,
};
use crate::utils::errors::AppError;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct StreamMetaGrpcClient {
    client: StreamMetaServiceClient<Channel>,
}

impl StreamMetaGrpcClient {
    pub async fn connect(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = StreamMetaServiceClient::connect(url).await?;
        Ok(Self { client })
    }

    pub async fn get_live_streams(
        &mut self,
        limit: i32,
        cursor: String,
    ) -> Result<GetLiveStreamsResponse, AppError> {
        let request = tonic::Request::new(GetLiveStreamsRequest { limit, cursor });
        let response = self.client.get_live_streams(request).await?;
        Ok(response.into_inner())
    }
}
