use anyhow::Result;
use async_trait::async_trait;
use fta_database::{entities::users, sea_orm, DbPool};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::domain::{User, UserRepository};

fn model_to_user(model: users::Model) -> User {
    User {
        id: model.id,
        email: model.email,
        password_hash: model.password_hash,
        first_name: model.first_name,
        last_name: model.last_name,
        email_verified: model.email_verified,
        two_factor_enabled: model.two_factor_enabled,
        two_factor_secret: model.two_factor_secret,
        oauth_provider: model.oauth_provider,
        oauth_provider_id: model.oauth_provider_id,
        last_login_at: model.last_login_at,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

fn user_to_active_model(user: &User) -> users::ActiveModel {
    users::ActiveModel {
        id: Set(user.id),
        email: Set(user.email.clone()),
        password_hash: Set(user.password_hash.clone()),
        first_name: Set(user.first_name.clone()),
        last_name: Set(user.last_name.clone()),
        email_verified: Set(user.email_verified),
        two_factor_enabled: Set(user.two_factor_enabled),
        two_factor_secret: Set(user.two_factor_secret.clone()),
        oauth_provider: Set(user.oauth_provider.clone()),
        oauth_provider_id: Set(user.oauth_provider_id.clone()),
        last_login_at: Set(user.last_login_at),
        created_at: Set(user.created_at),
        updated_at: Set(user.updated_at),
    }
}

#[derive(Clone, Debug)]
pub struct PostgresUserRepository {
    pool: DbPool,
}

impl PostgresUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<User> {
        let active_model = user_to_active_model(&user);
        let result = active_model.insert(&self.pool).await?;
        Ok(model_to_user(result))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>> {
        let result = users::Entity::find_by_id(*id).one(&self.pool).await?;
        Ok(result.map(model_to_user))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let result = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.pool)
            .await?;
        Ok(result.map(model_to_user))
    }

    async fn update(&self, user: User) -> Result<User> {
        let active_model = users::ActiveModel {
            id: Set(user.id),
            email: Set(user.email.clone()),
            password_hash: Set(user.password_hash.clone()),
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            email_verified: Set(user.email_verified),
            two_factor_enabled: Set(user.two_factor_enabled),
            two_factor_secret: Set(user.two_factor_secret.clone()),
            oauth_provider: Set(user.oauth_provider.clone()),
            oauth_provider_id: Set(user.oauth_provider_id.clone()),
            last_login_at: Set(user.last_login_at),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        };

        let result = active_model.update(&self.pool).await?;
        Ok(model_to_user(result))
    }

    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::UserFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>> {
        let mut query = users::Entity::find();

        // Apply filters
        if let Some(ref email) = filters.email {
            query = query.filter(users::Column::Email.contains(email));
        }
        if let Some(ref first_name) = filters.first_name {
            query = query.filter(users::Column::FirstName.contains(first_name));
        }
        if let Some(ref last_name) = filters.last_name {
            query = query.filter(users::Column::LastName.contains(last_name));
        }
        if let Some(verified) = filters.verified_only {
            query = query.filter(users::Column::EmailVerified.eq(verified));
        }
        if let Some(ref provider) = filters.oauth_provider {
            query = query.filter(users::Column::OauthProvider.eq(provider));
        }
        if let Some(created_after) = filters.created_after {
            query = query.filter(users::Column::CreatedAt.gte(created_after));
        }
        if let Some(created_before) = filters.created_before {
            query = query.filter(users::Column::CreatedAt.lte(created_before));
        }
        if let Some(two_factor) = filters.two_factor_enabled {
            query = query.filter(users::Column::TwoFactorEnabled.eq(two_factor));
        }

        // Apply sorting
        let order = if sort_order.to_lowercase() == "asc" {
            sea_orm::Order::Asc
        } else {
            sea_orm::Order::Desc
        };

        query = match sort_by {
            Some("email") => query.order_by(users::Column::Email, order),
            Some("first_name") => query.order_by(users::Column::FirstName, order),
            Some("last_name") => query.order_by(users::Column::LastName, order),
            Some("created_at") | None => query.order_by(users::Column::CreatedAt, order),
            Some("updated_at") => query.order_by(users::Column::UpdatedAt, order),
            Some(_) => query.order_by(users::Column::CreatedAt, order),
        };

        let results = query
            .paginate(&self.pool, limit as u64)
            .fetch_page((offset / limit.max(1)) as u64)
            .await?;

        Ok(results.into_iter().map(model_to_user).collect())
    }

    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::UserFilters,
    ) -> Result<i64> {
        let mut query = users::Entity::find();

        // Apply filters
        if let Some(ref email) = filters.email {
            query = query.filter(users::Column::Email.contains(email));
        }
        if let Some(ref first_name) = filters.first_name {
            query = query.filter(users::Column::FirstName.contains(first_name));
        }
        if let Some(ref last_name) = filters.last_name {
            query = query.filter(users::Column::LastName.contains(last_name));
        }
        if let Some(verified) = filters.verified_only {
            query = query.filter(users::Column::EmailVerified.eq(verified));
        }
        if let Some(ref provider) = filters.oauth_provider {
            query = query.filter(users::Column::OauthProvider.eq(provider));
        }
        if let Some(created_after) = filters.created_after {
            query = query.filter(users::Column::CreatedAt.gte(created_after));
        }
        if let Some(created_before) = filters.created_before {
            query = query.filter(users::Column::CreatedAt.lte(created_before));
        }
        if let Some(two_factor) = filters.two_factor_enabled {
            query = query.filter(users::Column::TwoFactorEnabled.eq(two_factor));
        }

        let count = query.count(&self.pool).await?;
        Ok(count as i64)
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        users::Entity::delete_by_id(*id).exec(&self.pool).await?;
        Ok(())
    }
}
