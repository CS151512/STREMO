use crate::models::domain::ChatMessage;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;

#[derive(Clone)]
pub struct PostgresBatcher {
    buffer: Arc<Mutex<Vec<ChatMessage>>>,
    pool: PgPool,
}

impl PostgresBatcher {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .connect(database_url)
            .await?;

        let batcher = Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            pool,
        };

        batcher.start_flush_worker();
        Ok(batcher)
    }

    pub async fn add_message(&self, msg: ChatMessage) {
        let mut buf = self.buffer.lock().await;
        buf.push(msg);
    }

    pub async fn get_history(&self, channel_id: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<ChatMessage>> {
        let channel_uuid = uuid::Uuid::parse_str(channel_id).unwrap_or_default();

        #[derive(sqlx::FromRow)]
        struct ChatMessageRow {
            id: uuid::Uuid,
            channel_id: uuid::Uuid,
            user_id: uuid::Uuid,
            message_text: String,
            created_at: chrono::NaiveDateTime,
        }

        let rows = sqlx::query_as::<_, ChatMessageRow>(
            r#"
            SELECT id, channel_id, user_id, message_text, created_at
            FROM chat_messages
            WHERE channel_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(channel_uuid)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let messages = rows.into_iter().map(|r| ChatMessage {
            id: r.id.to_string(),
            channel_id: r.channel_id.to_string(),
            user_id: r.user_id.to_string(),
            username: "".to_string(),
            text: r.message_text,
            timestamp: r.created_at.and_utc().timestamp_millis(),
        }).collect();

        Ok(messages)
    }

    fn start_flush_worker(&self) {
        let buffer_clone = self.buffer.clone();
        let pool_clone = self.pool.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(500));
            loop {
                interval.tick().await;

                let messages = {
                    let mut buf = buffer_clone.lock().await;
                    if buf.is_empty() {
                        continue;
                    }
                    std::mem::take(&mut *buf)
                };

                let len = messages.len();
                if let Err(e) = Self::flush_batch(&pool_clone, messages).await {
                    tracing::error!("Failed to flush {} messages to PostgreSQL: {}", len, e);
                } else {
                    tracing::debug!("Successfully inserted {} messages into DB", len);
                }
            }
        });
    }

    async fn flush_batch(pool: &PgPool, messages: Vec<ChatMessage>) -> Result<(), sqlx::Error> {
        let mut ids = Vec::with_capacity(messages.len());
        let mut channels = Vec::with_capacity(messages.len());
        let mut users = Vec::with_capacity(messages.len());
        let mut texts = Vec::with_capacity(messages.len());
        let mut timestamps = Vec::with_capacity(messages.len());

        for m in messages {
            let id = uuid::Uuid::parse_str(&m.id).unwrap_or_else(|_| uuid::Uuid::new_v4());
            let channel_id = uuid::Uuid::parse_str(&m.channel_id).unwrap_or_else(|_| uuid::Uuid::new_v4());
            let user_id = uuid::Uuid::parse_str(&m.user_id).unwrap_or_else(|_| uuid::Uuid::new_v4());
            let dt = chrono::DateTime::from_timestamp_millis(m.timestamp)
                .unwrap_or_else(|| chrono::Utc::now())
                .naive_utc();

            ids.push(id);
            channels.push(channel_id);
            users.push(user_id);
            texts.push(m.text);
            timestamps.push(dt);
        }

        sqlx::query(
            "INSERT INTO chat_messages (id, channel_id, user_id, message_text, created_at)
             SELECT * FROM UNNEST($1, $2, $3, $4, $5)"
        )
        .bind(ids)
        .bind(channels)
        .bind(users)
        .bind(texts)
        .bind(timestamps)
        .execute(pool)
        .await?;

        Ok(())
    }
}
