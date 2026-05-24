use crate::config::Config;
use std::env;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_config_from_env_defaults() {
    let _guard = ENV_LOCK.lock().unwrap();

    env::remove_var("PORT");
    env::remove_var("REDIS_URL");
    env::remove_var("AUTH_SERVICE_GRPC_URL");
    env::remove_var("STREAM_META_SERVICE_GRPC_URL");
    env::remove_var("JWT_SECRET");

    let config = Config::from_env();

    assert_eq!(config.port, 8000);
    assert_eq!(config.redis_url, "redis://127.0.0.1:6379");
    assert_eq!(config.auth_service_grpc_url, "http://127.0.0.1:50051");
    assert_eq!(config.stream_meta_grpc_url, "http://127.0.0.1:50052");
    assert_eq!(config.jwt_secret, "super_secret_key_for_local_dev");
}

#[test]
fn test_config_from_env_custom() {
    let _guard = ENV_LOCK.lock().unwrap();

    env::set_var("PORT", "9090");
    env::set_var("REDIS_URL", "redis://redis-cluster:6379");
    env::set_var("JWT_SECRET", "prod_secret_key");

    let config = Config::from_env();

    assert_eq!(config.port, 9090);
    assert_eq!(config.redis_url, "redis://redis-cluster:6379");
    assert_eq!(config.jwt_secret, "prod_secret_key");

    env::remove_var("PORT");
    env::remove_var("REDIS_URL");
    env::remove_var("JWT_SECRET");
}
