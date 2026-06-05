use super::super::db::PgRepo;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct StreamMeta {
    pub id: Uuid,
    pub streamer_id: Uuid,
    pub title: String,
}

impl PgRepo {
    pub async fn get_stream_meta(
        &self,
        stream_id: Uuid,
    ) -> Result<Option<StreamMeta>, sqlx::Error> {
        let meta = sqlx::query_as::<_, StreamMeta>(
            "SELECT id, streamer_id, title FROM streams WHERE id = $1",
        )
        .bind(stream_id)
        .fetch_optional(self.pool())
        .await?;
        Ok(meta)
    }
}
