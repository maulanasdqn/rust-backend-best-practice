use anyhow::Result;
use async_trait::async_trait;
use fta_database::DbPool;
use sqlx::Row;
use uuid::Uuid;

use crate::domain::{Account, AccountRepository, AccountType};

#[derive(Clone, Debug)]
pub struct PostgresAccountRepository {
    pool: DbPool,
}

impl PostgresAccountRepository {
    pub const fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for PostgresAccountRepository {
    async fn create(&self, account: Account) -> Result<Account> {
        let account_type_str = serde_json::to_string(&account.account_type)?;

        let result = sqlx::query(
      r"
      INSERT INTO accounts (id, user_id, name, account_type, balance, currency, is_active, created_at, updated_at)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
      RETURNING id, user_id, name, account_type, balance, currency, is_active, created_at, updated_at
      ",
    )
    .bind(account.id)
    .bind(account.user_id)
    .bind(&account.name)
    .bind(account_type_str)
    .bind(account.balance)
    .bind(&account.currency)
    .bind(account.is_active)
    .bind(account.created_at)
    .bind(account.updated_at)
    .fetch_one(&self.pool)
    .await?;

        let account_type: AccountType = serde_json::from_str(result.get("account_type"))?;

        Ok(Account {
            id: result.get("id"),
            user_id: result.get("user_id"),
            name: result.get("name"),
            account_type,
            balance: result.get("balance"),
            currency: result.get("currency"),
            is_active: result.get("is_active"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Account>> {
        let result = sqlx::query(
            r"
      SELECT id, user_id, name, account_type, balance, currency, is_active, created_at, updated_at
      FROM accounts
      WHERE id = $1
      ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| {
            let account_type: AccountType =
                serde_json::from_str(row.get("account_type")).unwrap_or(AccountType::Checking);

            Account {
                id: row.get("id"),
                user_id: row.get("user_id"),
                name: row.get("name"),
                account_type,
                balance: row.get("balance"),
                currency: row.get("currency"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }))
    }

    async fn find_by_user_id(
        &self,
        user_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Account>> {
        let results = sqlx::query(
            r"
      SELECT id, user_id, name, account_type, balance, currency, is_active, created_at, updated_at
      FROM accounts
      WHERE user_id = $1
      ORDER BY created_at DESC
      LIMIT $2 OFFSET $3
      ",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|row| {
                let account_type: AccountType =
                    serde_json::from_str(row.get("account_type")).unwrap_or(AccountType::Checking);

                Account {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    account_type,
                    balance: row.get("balance"),
                    currency: row.get("currency"),
                    is_active: row.get("is_active"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            })
            .collect())
    }

    async fn count_by_user_id(&self, user_id: &Uuid) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            r"
      SELECT COUNT(*) FROM accounts WHERE user_id = $1
      ",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    async fn update(&self, account: Account) -> Result<Account> {
        let account_type_str = serde_json::to_string(&account.account_type)?;

        let result = sqlx::query(
      r"
      UPDATE accounts
      SET name = $2, account_type = $3, balance = $4, currency = $5, is_active = $6, updated_at = $7
      WHERE id = $1
      RETURNING id, user_id, name, account_type, balance, currency, is_active, created_at, updated_at
      ",
    )
    .bind(account.id)
    .bind(&account.name)
    .bind(account_type_str)
    .bind(account.balance)
    .bind(&account.currency)
    .bind(account.is_active)
    .bind(account.updated_at)
    .fetch_one(&self.pool)
    .await?;

        let account_type: AccountType = serde_json::from_str(result.get("account_type"))?;

        Ok(Account {
            id: result.get("id"),
            user_id: result.get("user_id"),
            name: result.get("name"),
            account_type,
            balance: result.get("balance"),
            currency: result.get("currency"),
            is_active: result.get("is_active"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
