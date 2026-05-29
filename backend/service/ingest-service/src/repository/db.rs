use crate::models::domain::Stream;

pub trait StreamRepository: Send + Sync {
    async fn get_stream(&self, stream_id: &uuid::Uuid) -> anyhow::Result<Option<Stream>>;
    //дописать потом
    // async fn create_stream(&self, stream: &Stream) -> anyhow::Result<()>;
}
