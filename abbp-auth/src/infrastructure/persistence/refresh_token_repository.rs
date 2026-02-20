//! PostgreSQL implementation of the refresh token repository.

use async_trait::async_trait;
use chrono::Utc;
use abbp_database::{entities::refresh_tokens, sea_orm, DbPool};
use abbp_errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use uuid::Uuid;

use crate::domain::{RefreshToken, RefreshTokenRepository};

fn model_to_refresh_token(model: refresh_tokens::Model) -> RefreshToken {
    RefreshToken {
        id: model.id,
        user_id: model.user_id,
        token: model.token,
        expires_at: model.expires_at,
        created_at: model.created_at,
    }
}

/// Converts a database error to an application error.
fn db_err(e: impl std::fmt::Display) -> AppError {
    AppError::InternalError(format!("Database error: {e}"))
}

#[derive(Clone, Debug)]
pub struct PostgresRefreshTokenRepository {
    pool: DbPool,
}

impl PostgresRefreshTokenRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn create(&self, refresh_token: RefreshToken) -> Result<RefreshToken, AppError> {
        let active_model = refresh_tokens::ActiveModel {
            id: Set(refresh_token.id),
            user_id: Set(refresh_token.user_id),
            token: Set(refresh_token.token.clone()),
            expires_at: Set(refresh_token.expires_at),
            created_at: Set(refresh_token.created_at),
        };

        let result = active_model.insert(&self.pool).await.map_err(db_err)?;
        Ok(model_to_refresh_token(result))
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError> {
        let result = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::Token.eq(token))
            .one(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(result.map(model_to_refresh_token))
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<RefreshToken>, AppError> {
        let results = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::UserId.eq(*user_id))
            .order_by_desc(refresh_tokens::Column::CreatedAt)
            .all(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(results.into_iter().map(model_to_refresh_token).collect())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        refresh_tokens::Entity::delete_by_id(*id)
            .exec(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), AppError> {
        refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::UserId.eq(*user_id))
            .exec(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_by_token(&self, token: &str) -> Result<(), AppError> {
        refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::Token.eq(token))
            .exec(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, AppError> {
        let result = refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::ExpiresAt.lt(Utc::now()))
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
    async fn test_create_and_find_refresh_token() {}

    #[ignore = "Requires test database"]
    #[tokio::test]
    async fn test_delete_expired_tokens() {}
}
