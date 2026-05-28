use lazy_static::lazy_static;
use prometheus::{IntCounter, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref INGESTED_BYTES: IntCounter =
        IntCounter::new("ingested_bytes", "Total ingested bytes").unwrap();
}

pub fn registry_metrics() {
    REGISTRY.register(Box::new(INGESTED_BYTES.clone())).unwrap();
}

pub async fn metrics_handler() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
