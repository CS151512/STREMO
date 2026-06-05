use crate::models::domain::PingRow;
use crate::service::aggregator::AnalyticsAggregator;
use pb::analytics_service_server::AnalyticsService;
use pb::{GetSummaryRequest, GetSummaryResponse, ReportPingRequest, ReportPingResponse};
use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("stremo.analytics.v1");
}

pub struct GrpcServerImpl {
    aggregator: AnalyticsAggregator,
}

impl GrpcServerImpl {
    pub fn new(aggregator: AnalyticsAggregator) -> Self {
        Self { aggregator }
    }
}

#[tonic::async_trait]
impl AnalyticsService for GrpcServerImpl {
    async fn report_ping(
        &self,
        request: Request<ReportPingRequest>,
    ) -> Result<Response<ReportPingResponse>, Status> {
        let req = request.into_inner();

        let ping = PingRow {
            stream_id: req.stream_id,
            client_ip: req.client_ip,
            timestamp: req.timestamp,
        };

        match self.aggregator.process_ping(ping).await {
            Ok(_) => Ok(Response::new(ReportPingResponse { success: true })),
            Err(e) => {
                tracing::error!("Error processing ping: {}", e);
                Err(Status::internal("Failed to process ping"))
            }
        }
    }

    async fn get_summary(
        &self,
        request: Request<GetSummaryRequest>,
    ) -> Result<Response<GetSummaryResponse>, Status> {
        let req = request.into_inner();

        match self.aggregator.get_stream_summary(&req.stream_id).await {
            Ok(summary) => Ok(Response::new(GetSummaryResponse {
                stream_id: req.stream_id,
                current_ccv: summary.current_ccv,
                peak_ccv: summary.peak_ccv as i32,
                total_unique_views: summary.total_unique_views as i64,
            })),
            Err(e) => {
                tracing::error!(
                    "Failed to fetch stream summary for {}: {}",
                    req.stream_id,
                    e
                );
                Err(Status::internal("Failed to calculate stream summary"))
            }
        }
    }
}
