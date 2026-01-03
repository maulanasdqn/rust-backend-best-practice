use async_trait::async_trait;
use chrono::Utc;
use fta_database::{entities::email_verifications, sea_orm, DbPool};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use uuid::Uuid;

use crate::domain::{EmailVerification, EmailVerificationRepository};

fn model_to_email_verification(model: email_verifications::Model) -> EmailVerification {
    EmailVerification {
        id: model.id,
        user_id: model.user_id,
        otp_code: model.otp_code,
        expires_at: model.expires_at,
        created_at: model.created_at,
    }
}

#[derive(Clone, Debug)]
pub struct PostgresEmailVerificationRepository {
    pool: DbPool,
}

impl PostgresEmailVerificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmailVerificationRepository for PostgresEmailVerificationRepository {
    async fn create(&self, verification: EmailVerification) -> anyhow::Result<EmailVerification> {
        let active_model = email_verifications::ActiveModel {
            id: Set(verification.id),
            user_id: Set(verification.user_id),
            otp_code: Set(verification.otp_code.clone()),
            expires_at: Set(verification.expires_at),
            created_at: Set(verification.created_at),
        };

        let result = active_model.insert(&self.pool).await?;
        Ok(model_to_email_verification(result))
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Option<EmailVerification>> {
        let result = email_verifications::Entity::find()
            .filter(email_verifications::Column::UserId.eq(*user_id))
            .order_by_desc(email_verifications::Column::CreatedAt)
            .one(&self.pool)
            .await?;

        Ok(result.map(model_to_email_verification))
    }

    async fn delete(&self, id: &Uuid) -> anyhow::Result<()> {
        email_verifications::Entity::delete_by_id(*id)
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<()> {
        email_verifications::Entity::delete_many()
            .filter(email_verifications::Column::UserId.eq(*user_id))
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_expired(&self) -> anyhow::Result<u64> {
        let result = email_verifications::Entity::delete_many()
            .filter(email_verifications::Column::ExpiresAt.lt(Utc::now()))
            .exec(&self.pool)
            .await?;

        Ok(result.rows_affected)
    }
}

#[cfg(test)]
mod tests {

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_create_and_find_email_verification() {}

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_delete_expired_verifications() {}
}
