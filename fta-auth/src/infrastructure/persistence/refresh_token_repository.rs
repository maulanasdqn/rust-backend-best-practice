use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{RefreshToken, RefreshTokenRepository};

#[derive(Clone, Debug)]
pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
}

impl PostgresRefreshTokenRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn create(&self, refresh_token: RefreshToken) -> anyhow::Result<RefreshToken> {
        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, token, expires_at, created_at
            "#,
            refresh_token.id,
            refresh_token.user_id,
            refresh_token.token,
            refresh_token.expires_at,
            refresh_token.created_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    async fn find_by_token(&self, token: &str) -> anyhow::Result<Option<RefreshToken>> {
        let result = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT id, user_id, token, expires_at, created_at
            FROM refresh_tokens
            WHERE token = $1
            "#,
            token,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Vec<RefreshToken>> {
        let tokens = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT id, user_id, token, expires_at, created_at
            FROM refresh_tokens
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }

    async fn delete(&self, id: &Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM refresh_tokens
            WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM refresh_tokens
            WHERE user_id = $1
            "#,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_by_token(&self, token: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM refresh_tokens
            WHERE token = $1
            "#,
            token,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_expired(&self) -> anyhow::Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM refresh_tokens
            WHERE expires_at < NOW()
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_create_and_find_refresh_token() {}

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_delete_expired_tokens() {}
}
