#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub redis_url: String,
    pub auth_service_grpc_url: String,
    pub stream_meta_grpc_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("PORT must be a number"),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            auth_service_grpc_url: std::env::var("AUTH_SERVICE_GRPC_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:50051".to_string()),
            stream_meta_grpc_url: std::env::var("STREAM_META_SERVICE_GRPC_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:50052".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "super_secret_key_for_local_dev".to_string()),
        }
    }
}
