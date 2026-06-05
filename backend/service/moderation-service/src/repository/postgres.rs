use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct PostgresAuditRepo {
    pool: PgPool,
}

impl PostgresAuditRepo {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn log_ban(&self, user_id: &str, channel_id: &str, reason: &str, moderator_id: &str) -> anyhow::Result<()> {
        let id = Uuid::new_v4();
        let u_id = Uuid::parse_str(user_id).unwrap_or_default();
        let c_id = Uuid::parse_str(channel_id).unwrap_or_default();
        let m_id = moderator_id.to_string();
        let now = Utc::now().naive_utc();

        sqlx::query(
            "INSERT INTO moderation_audit_log (id, user_id, channel_id, action, reason, moderator_id, created_at)
             VALUES ($1, $2, $3, 'BAN', $4, $5, $6)"
        )
        .bind(id)
        .bind(u_id)
        .bind(c_id)
        .bind(reason)
        .bind(m_id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
