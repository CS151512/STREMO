use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub http_port: u16,
    pub grpc_port: u16,
    pub clickhouse_url: String,
    pub redis_url: String,
    pub kafka_brokers: String,
    pub environment: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        let config = Config::builder()
            .set_default("http_port", 8080)?
            .set_default("grpc_port", 50051)?
            .set_default("clickhouse_url", "http://localhost:8123")?
            .set_default("redis_url", "redis://localhost:6379")?
            .set_default("kafka_brokers", "localhost:9092")?
            .set_default("environment", "development")?
            .add_source(Environment::default().separator("_"))
            .build()?;
        config.try_deserialize()
    }
}
