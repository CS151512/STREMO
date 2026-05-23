pub mod auth_client;
pub mod stream_meta_client;

pub mod pb {
    pub mod auth {
        tonic::include_proto!("stremo.auth.v1");
    }
    pub mod stream_meta {
        tonic::include_proto!("stremo.stream_meta.v1");
    }
}
