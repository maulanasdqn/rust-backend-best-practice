use async_trait::async_trait;
use chrono::Utc;
use fta_database::{entities::refresh_tokens, sea_orm, DbPool};
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
    async fn create(&self, refresh_token: RefreshToken) -> anyhow::Result<RefreshToken> {
        let active_model = refresh_tokens::ActiveModel {
            id: Set(refresh_token.id),
            user_id: Set(refresh_token.user_id),
            token: Set(refresh_token.token.clone()),
            expires_at: Set(refresh_token.expires_at),
            created_at: Set(refresh_token.created_at),
        };

        let result = active_model.insert(&self.pool).await?;
        Ok(model_to_refresh_token(result))
    }

    async fn find_by_token(&self, token: &str) -> anyhow::Result<Option<RefreshToken>> {
        let result = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::Token.eq(token))
            .one(&self.pool)
            .await?;

        Ok(result.map(model_to_refresh_token))
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Vec<RefreshToken>> {
        let results = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::UserId.eq(*user_id))
            .order_by_desc(refresh_tokens::Column::CreatedAt)
            .all(&self.pool)
            .await?;

        Ok(results.into_iter().map(model_to_refresh_token).collect())
    }

    async fn delete(&self, id: &Uuid) -> anyhow::Result<()> {
        refresh_tokens::Entity::delete_by_id(*id)
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<()> {
        refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::UserId.eq(*user_id))
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_by_token(&self, token: &str) -> anyhow::Result<()> {
        refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::Token.eq(token))
            .exec(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_expired(&self) -> anyhow::Result<u64> {
        let result = refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::ExpiresAt.lt(Utc::now()))
            .exec(&self.pool)
            .await?;

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
