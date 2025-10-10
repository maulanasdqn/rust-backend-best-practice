use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{EmailVerification, EmailVerificationRepository};

/// Postgres implementation of `EmailVerificationRepository`
#[derive(Clone, Debug)]
pub struct PostgresEmailVerificationRepository {
    pool: PgPool,
}

impl PostgresEmailVerificationRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmailVerificationRepository for PostgresEmailVerificationRepository {
    async fn create(&self, verification: EmailVerification) -> anyhow::Result<EmailVerification> {
        let result = sqlx::query_as!(
            EmailVerification,
            r#"
            INSERT INTO email_verifications (id, user_id, otp_code, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, otp_code, expires_at, created_at
            "#,
            verification.id,
            verification.user_id,
            verification.otp_code,
            verification.expires_at,
            verification.created_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Option<EmailVerification>> {
        let result = sqlx::query_as!(
            EmailVerification,
            r#"
            SELECT id, user_id, otp_code, expires_at, created_at
            FROM email_verifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, id: &Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM email_verifications
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
            DELETE FROM email_verifications
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
            DELETE FROM email_verifications
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
    async fn test_create_and_find_email_verification() {
        // This test requires a real database connection
        // Implement when you have test database setup
    }

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_delete_expired_verifications() {
        // This test requires a real database connection
        // Implement when you have test database setup
    }
}
