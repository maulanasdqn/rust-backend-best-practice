use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{PasswordResetToken, PasswordResetTokenRepository};

/// Postgres implementation of `PasswordResetTokenRepository`
#[derive(Clone, Debug)]
pub struct PostgresPasswordResetTokenRepository {
    pool: PgPool,
}

impl PostgresPasswordResetTokenRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PasswordResetTokenRepository for PostgresPasswordResetTokenRepository {
    async fn create(&self, token: PasswordResetToken) -> anyhow::Result<PasswordResetToken> {
        let result = sqlx::query_as!(
            PasswordResetToken,
            r#"
            INSERT INTO password_reset_tokens (id, user_id, token, expires_at, used, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, token, expires_at, used, created_at
            "#,
            token.id,
            token.user_id,
            token.token,
            token.expires_at,
            token.used,
            token.created_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_token(&self, token: &str) -> anyhow::Result<Option<PasswordResetToken>> {
        let result = sqlx::query_as!(
            PasswordResetToken,
            r#"
            SELECT id, user_id, token, expires_at, used, created_at
            FROM password_reset_tokens
            WHERE token = $1
            "#,
            token,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Vec<PasswordResetToken>> {
        let tokens = sqlx::query_as!(
            PasswordResetToken,
            r#"
            SELECT id, user_id, token, expires_at, used, created_at
            FROM password_reset_tokens
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }

    async fn update(&self, token: PasswordResetToken) -> anyhow::Result<PasswordResetToken> {
        let result = sqlx::query_as!(
            PasswordResetToken,
            r#"
            UPDATE password_reset_tokens
            SET used = $1, expires_at = $2
            WHERE id = $3
            RETURNING id, user_id, token, expires_at, used, created_at
            "#,
            token.used,
            token.expires_at,
            token.id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, id: &Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM password_reset_tokens
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
            DELETE FROM password_reset_tokens
            WHERE user_id = $1
            "#,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_expired(&self) -> anyhow::Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM password_reset_tokens
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
    // Note: These tests require a test database
    // You would typically use sqlx::test macro for integration tests

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_create_and_find_password_reset_token() {
        // This test requires a real database connection
        // Implement when you have test database setup
    }

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_mark_token_as_used() {
        // This test requires a real database connection
        // Implement when you have test database setup
    }

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_delete_expired_tokens() {
        // This test requires a real database connection
        // Implement when you have test database setup
    }
}
