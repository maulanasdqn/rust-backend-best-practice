use anyhow::Result;
use async_trait::async_trait;
use fta_database::DbPool;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::domain::{User, UserRepository};

const USER_COLUMNS: &str = "id, email, password_hash, first_name, last_name, email_verified, \
    two_factor_enabled, two_factor_secret, oauth_provider, oauth_provider_id, \
    last_login_at, created_at, updated_at";

fn row_to_user(row: PgRow) -> User {
    User {
        id: row.get("id"),
        email: row.get("email"),
        password_hash: row.get("password_hash"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        email_verified: row.get("email_verified"),
        two_factor_enabled: row.get("two_factor_enabled"),
        two_factor_secret: row.get("two_factor_secret"),
        oauth_provider: row.get("oauth_provider"),
        oauth_provider_id: row.get("oauth_provider_id"),
        last_login_at: row.get("last_login_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

#[derive(Clone, Debug)]
pub struct PostgresUserRepository {
    pool: DbPool,
}

impl PostgresUserRepository {
    pub const fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<User> {
        let query = format!(
            "INSERT INTO users ({USER_COLUMNS}) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) \
             RETURNING {USER_COLUMNS}"
        );

        let result = sqlx::query(&query)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(user.email_verified)
            .bind(user.two_factor_enabled)
            .bind(&user.two_factor_secret)
            .bind(&user.oauth_provider)
            .bind(&user.oauth_provider_id)
            .bind(user.last_login_at)
            .bind(user.created_at)
            .bind(user.updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_user(result))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>> {
        let query = format!("SELECT {USER_COLUMNS} FROM users WHERE id = $1");

        let result = sqlx::query(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(row_to_user))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let query = format!("SELECT {USER_COLUMNS} FROM users WHERE email = $1");

        let result = sqlx::query(&query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(row_to_user))
    }

    async fn update(&self, user: User) -> Result<User> {
        let query = format!(
            "UPDATE users SET email = $2, password_hash = $3, first_name = $4, last_name = $5, \
             email_verified = $6, two_factor_enabled = $7, two_factor_secret = $8, \
             oauth_provider = $9, oauth_provider_id = $10, last_login_at = $11, updated_at = $12 \
             WHERE id = $1 RETURNING {USER_COLUMNS}"
        );

        let result = sqlx::query(&query)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(user.email_verified)
            .bind(user.two_factor_enabled)
            .bind(&user.two_factor_secret)
            .bind(&user.oauth_provider)
            .bind(&user.oauth_provider_id)
            .bind(user.last_login_at)
            .bind(user.updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_user(result))
    }

    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::UserFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>> {
        let mut query_builder =
            sqlx::QueryBuilder::new(format!("SELECT {USER_COLUMNS} FROM users WHERE 1=1"));

        apply_filters(&mut query_builder, filters);

        if let Some(sort_field) = sort_by {
            query_builder.push(format!(" ORDER BY {sort_field} {sort_order}"));
        } else {
            query_builder.push(" ORDER BY created_at DESC");
        }

        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        let results = query_builder.build().fetch_all(&self.pool).await?;

        Ok(results.into_iter().map(row_to_user).collect())
    }

    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::UserFilters,
    ) -> Result<i64> {
        let mut query_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM users WHERE 1=1");

        apply_filters(&mut query_builder, filters);

        let result: (i64,) = query_builder.build_query_as().fetch_one(&self.pool).await?;

        Ok(result.0)
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

fn apply_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Postgres>,
    filters: &'a crate::infrastructure::http::filters::UserFilters,
) {
    if let Some(ref email) = filters.email {
        query_builder.push(" AND email ILIKE ");
        query_builder.push_bind(format!("%{email}%"));
    }

    if let Some(ref first_name) = filters.first_name {
        query_builder.push(" AND first_name ILIKE ");
        query_builder.push_bind(format!("%{first_name}%"));
    }

    if let Some(ref last_name) = filters.last_name {
        query_builder.push(" AND last_name ILIKE ");
        query_builder.push_bind(format!("%{last_name}%"));
    }

    if let Some(verified) = filters.verified_only {
        query_builder.push(" AND email_verified = ");
        query_builder.push_bind(verified);
    }

    if let Some(ref provider) = filters.oauth_provider {
        query_builder.push(" AND oauth_provider = ");
        query_builder.push_bind(provider);
    }

    if let Some(created_after) = filters.created_after {
        query_builder.push(" AND created_at >= ");
        query_builder.push_bind(created_after);
    }

    if let Some(created_before) = filters.created_before {
        query_builder.push(" AND created_at <= ");
        query_builder.push_bind(created_before);
    }

    if let Some(two_factor) = filters.two_factor_enabled {
        query_builder.push(" AND two_factor_enabled = ");
        query_builder.push_bind(two_factor);
    }
}
