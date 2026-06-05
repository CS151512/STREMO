use crate::service::moderator::ModeratorManager;
use pb::moderation_service_server::ModerationService;
use pb::{CheckSpamRequest, CheckSpamResponse, BanUserRequest, BanUserResponse};
use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("stremo.moderation.v1");
}

pub struct GrpcServerImpl {
    manager: ModeratorManager,
}

impl GrpcServerImpl {
    pub fn new(manager: ModeratorManager) -> Self {
        Self { manager }
    }
}

#[tonic::async_trait]
impl ModerationService for GrpcServerImpl {
    async fn check_spam(
        &self,
        request: Request<CheckSpamRequest>,
    ) -> Result<Response<CheckSpamResponse>, Status> {
        let req = request.into_inner();

        match self.manager.check_spam(&req.message_text, &req.user_id, &req.channel_id).await {
            Ok(result) => Ok(Response::new(CheckSpamResponse {
                is_spam: result.is_spam,
                confidence_score: result.confidence,
                reason: result.reason,
            })),
            Err(e) => {
                tracing::error!("Error checking spam: {}", e);
                Err(Status::internal("Failed to check spam"))
            }
        }
    }

    async fn ban_user(
        &self,
        request: Request<BanUserRequest>,
    ) -> Result<Response<BanUserResponse>, Status> {
        let req = request.into_inner();

        match self.manager.ban_user(&req.user_id, &req.channel_id, &req.reason, &req.moderator_id).await {
            Ok(_) => Ok(Response::new(BanUserResponse { success: true })),
            Err(e) => {
                tracing::error!("Failed to ban user {}: {}", req.user_id, e);
                Err(Status::internal("Failed to execute ban process"))
            }
        }
    }
}
