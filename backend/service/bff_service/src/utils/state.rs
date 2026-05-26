use crate::{config, grpc_clients};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub struct AppState {
    pub config: config::Config,
    pub redis_pool: Pool<RedisConnectionManager>,
    pub auth_grpc_client: grpc_clients::auth_client::AuthGrpcClient,
    pub stream_meta_grpc_client: grpc_clients::stream_meta_client::StreamMetaGrpcClient,
}
