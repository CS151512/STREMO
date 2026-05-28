use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tcp_port: u16,
    pub auth_service_grpc_url: String,
    pub kafka_brokers: String,
    pub hls_output_dir: String,
    pub ffmpeg_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            tcp_port: std::env::var("TCP_PORT")
                .unwrap_or_else(|_| "1935".to_string())
                .parse()
                .unwrap(),
            auth_service_grpc_url: std::env::var("AUTH_GRPC_URL")
                .unwrap_or_else(|_| "http://localhost:50051".to_string()),
            kafka_brokers: std::env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string()),
            hls_output_dir: std::env::var("HLS_OUTPUT_DIR")
                .unwrap_or_else(|_| "/tmp/hls".to_string()),
            ffmpeg_path: std::env::var("FFMPEG_PATH").unwrap_or_else(|_| "ffmpeg".to_string()),
        }
    }
}
