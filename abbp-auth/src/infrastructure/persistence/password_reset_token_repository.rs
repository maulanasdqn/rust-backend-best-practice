//! PostgreSQL implementation of the password reset token repository.

use async_trait::async_trait;
use chrono::Utc;
use abbp_database::{entities::password_reset_tokens, sea_orm, DbPool};
use abbp_errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use uuid::Uuid;

use crate::domain::{PasswordResetToken, PasswordResetTokenRepository};

fn model_to_password_reset_token(model: password_reset_tokens::Model) -> PasswordResetToken {
    PasswordResetToken {
        id: model.id,
        user_id: model.user_id,
        token: model.token,
        expires_at: model.expires_at,
        used: model.used,
        created_at: model.created_at,
    }
}

/// Converts a database error to an application error.
fn db_err(e: impl std::fmt::Display) -> AppError {
    AppError::InternalError(format!("Database error: {e}"))
}

#[derive(Clone, Debug)]
pub struct PostgresPasswordResetTokenRepository {
    pool: DbPool,
}

impl PostgresPasswordResetTokenRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PasswordResetTokenRepository for PostgresPasswordResetTokenRepository {
    async fn create(&self, token: PasswordResetToken) -> Result<PasswordResetToken, AppError> {
        let active_model = password_reset_tokens::ActiveModel {
            id: Set(token.id),
            user_id: Set(token.user_id),
            token: Set(token.token.clone()),
            expires_at: Set(token.expires_at),
            used: Set(token.used),
            created_at: Set(token.created_at),
        };

        let result = active_model.insert(&self.pool).await.map_err(db_err)?;
        Ok(model_to_password_reset_token(result))
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<PasswordResetToken>, AppError> {
        let result = password_reset_tokens::Entity::find()
            .filter(password_reset_tokens::Column::Token.eq(token))
            .one(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(result.map(model_to_password_reset_token))
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<PasswordResetToken>, AppError> {
        let results = password_reset_tokens::Entity::find()
            .filter(password_reset_tokens::Column::UserId.eq(*user_id))
            .order_by_desc(password_reset_tokens::Column::CreatedAt)
            .all(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(results
            .into_iter()
            .map(model_to_password_reset_token)
            .collect())
    }

    async fn update(&self, token: PasswordResetToken) -> Result<PasswordResetToken, AppError> {
        let active_model = password_reset_tokens::ActiveModel {
            id: Set(token.id),
            user_id: Set(token.user_id),
            token: Set(token.token.clone()),
            expires_at: Set(token.expires_at),
            used: Set(token.used),
            created_at: Set(token.created_at),
        };

        let result = active_model.update(&self.pool).await.map_err(db_err)?;
        Ok(model_to_password_reset_token(result))
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        password_reset_tokens::Entity::delete_by_id(*id)
            .exec(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), AppError> {
        password_reset_tokens::Entity::delete_many()
            .filter(password_reset_tokens::Column::UserId.eq(*user_id))
            .exec(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, AppError> {
        let result = password_reset_tokens::Entity::delete_many()
            .filter(password_reset_tokens::Column::ExpiresAt.lt(Utc::now()))
            .exec(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(result.rows_affected)
    }
}

#[cfg(test)]
mod tests {

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_create_and_find_password_reset_token() {}

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_mark_token_as_used() {}

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_delete_expired_tokens() {}
}
