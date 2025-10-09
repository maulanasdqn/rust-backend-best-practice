use anyhow::Result;
use async_trait::async_trait;
use fta_database::DbPool;
use sqlx::Row;
use uuid::Uuid;

use crate::domain::{User, UserRepository};

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
        let result = sqlx::query(
            r"
      INSERT INTO users (id, email, password_hash, first_name, last_name, email_verified, two_factor_enabled, two_factor_secret, oauth_provider, oauth_provider_id, last_login_at, created_at, updated_at)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
      RETURNING id, email, password_hash, first_name, last_name, email_verified, two_factor_enabled, two_factor_secret, oauth_provider, oauth_provider_id, last_login_at, created_at, updated_at
      ",
        )
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

        Ok(User {
            id: result.get("id"),
            email: result.get("email"),
            password_hash: result.get("password_hash"),
            first_name: result.get("first_name"),
            last_name: result.get("last_name"),
            email_verified: result.get("email_verified"),
            two_factor_enabled: result.get("two_factor_enabled"),
            two_factor_secret: result.get("two_factor_secret"),
            oauth_provider: result.get("oauth_provider"),
            oauth_provider_id: result.get("oauth_provider_id"),
            last_login_at: result.get("last_login_at"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>> {
        let result = sqlx::query(
            r"
      SELECT id, email, password_hash, first_name, last_name, email_verified, two_factor_enabled, two_factor_secret, oauth_provider, oauth_provider_id, last_login_at, created_at, updated_at
      FROM users
      WHERE id = $1
      ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| User {
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
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let result = sqlx::query(
            r"
      SELECT id, email, password_hash, first_name, last_name, email_verified, two_factor_enabled, two_factor_secret, oauth_provider, oauth_provider_id, last_login_at, created_at, updated_at
      FROM users
      WHERE email = $1
      ",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| User {
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
        }))
    }

    async fn update(&self, user: User) -> Result<User> {
        let result = sqlx::query(
            r"
      UPDATE users
      SET email = $2, password_hash = $3, first_name = $4, last_name = $5, email_verified = $6, two_factor_enabled = $7, two_factor_secret = $8, oauth_provider = $9, oauth_provider_id = $10, last_login_at = $11, updated_at = $12
      WHERE id = $1
      RETURNING id, email, password_hash, first_name, last_name, email_verified, two_factor_enabled, two_factor_secret, oauth_provider, oauth_provider_id, last_login_at, created_at, updated_at
      ",
        )
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

        Ok(User {
            id: result.get("id"),
            email: result.get("email"),
            password_hash: result.get("password_hash"),
            first_name: result.get("first_name"),
            last_name: result.get("last_name"),
            email_verified: result.get("email_verified"),
            two_factor_enabled: result.get("two_factor_enabled"),
            two_factor_secret: result.get("two_factor_secret"),
            oauth_provider: result.get("oauth_provider"),
            oauth_provider_id: result.get("oauth_provider_id"),
            last_login_at: result.get("last_login_at"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<User>> {
        let results = sqlx::query(
            r"
      SELECT id, email, password_hash, first_name, last_name, email_verified, two_factor_enabled, two_factor_secret, oauth_provider, oauth_provider_id, last_login_at, created_at, updated_at
      FROM users
      ORDER BY created_at DESC
      LIMIT $1 OFFSET $2
      ",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|row| User {
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
            })
            .collect())
    }

    async fn count_all(&self) -> Result<i64> {
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;
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
