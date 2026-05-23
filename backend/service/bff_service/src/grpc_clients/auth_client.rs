use super::pb::auth::{
    auth_service_client::AuthServiceClient, LoginRequest, LoginResponse, RegisterRequest,
    RegisterResponse,
};
use crate::utils::errors::AppError;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct AuthGrpcClient {
    client: AuthServiceClient<Channel>,
}

impl AuthGrpcClient {
    pub async fn connect(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AuthServiceClient::connect(url).await?;
        Ok(Self { client })
    }

    pub async fn login(
        &mut self,
        email: String,
        password: String,
    ) -> Result<LoginResponse, AppError> {
        let request = tonic::Request::new(LoginRequest { email, password });
        let response = self.client.login(request).await?;
        Ok(response.into_inner())
    }

    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<RegisterResponse, AppError> {
        let request = tonic::Request::new(RegisterRequest {
            username,
            email,
            password,
        });
        let response = self.client.register(request).await?;
        Ok(response.into_inner())
    }
}
