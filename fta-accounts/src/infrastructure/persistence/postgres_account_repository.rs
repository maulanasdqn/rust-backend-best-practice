use anyhow::Result;
use async_trait::async_trait;
use fta_database::DbPool;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::domain::{Account, AccountRepository, AccountType};
use crate::infrastructure::http::filters::AccountFilters;

const ACCOUNT_COLUMNS: &str =
    "id, user_id, name, account_type, balance, currency, is_active, created_at, updated_at";

fn row_to_account(row: PgRow) -> Account {
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
}

fn apply_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Postgres>,
    filters: &'a AccountFilters,
) {
    if let Some(user_id) = filters.user_id {
        query_builder.push(" AND user_id = ");
        query_builder.push_bind(user_id);
    }

    if let Some(ref account_type) = filters.account_type {
        let account_type_str = serde_json::to_string(account_type).unwrap_or_default();
        query_builder.push(" AND account_type = ");
        query_builder.push_bind(account_type_str);
    }

    if let Some(ref currency) = filters.currency {
        query_builder.push(" AND currency = ");
        query_builder.push_bind(currency);
    }

    if let Some(is_active) = filters.is_active {
        query_builder.push(" AND is_active = ");
        query_builder.push_bind(is_active);
    }

    if let Some(min_balance) = filters.min_balance {
        query_builder.push(" AND balance >= ");
        query_builder.push_bind(min_balance);
    }

    if let Some(max_balance) = filters.max_balance {
        query_builder.push(" AND balance <= ");
        query_builder.push_bind(max_balance);
    }

    if let Some(ref name) = filters.name {
        query_builder.push(" AND name ILIKE ");
        query_builder.push_bind(format!("%{name}%"));
    }
}

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
        let query = format!(
            "INSERT INTO accounts ({ACCOUNT_COLUMNS}) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
             RETURNING {ACCOUNT_COLUMNS}"
        );

        let result = sqlx::query(&query)
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

        Ok(row_to_account(result))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Account>> {
        let query = format!("SELECT {ACCOUNT_COLUMNS} FROM accounts WHERE id = $1");

        let result = sqlx::query(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(row_to_account))
    }

    async fn find_all(
        &self,
        filters: &AccountFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Account>> {
        let mut query_builder =
            sqlx::QueryBuilder::new(format!("SELECT {ACCOUNT_COLUMNS} FROM accounts WHERE 1=1"));

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

        Ok(results.into_iter().map(row_to_account).collect())
    }

    async fn count_all(&self, filters: &AccountFilters) -> Result<i64> {
        let mut query_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM accounts WHERE 1=1");

        apply_filters(&mut query_builder, filters);

        let result: (i64,) = query_builder.build_query_as().fetch_one(&self.pool).await?;

        Ok(result.0)
    }

    async fn update(&self, account: Account) -> Result<Account> {
        let account_type_str = serde_json::to_string(&account.account_type)?;
        let query = format!(
            "UPDATE accounts SET name = $2, account_type = $3, balance = $4, \
             currency = $5, is_active = $6, updated_at = $7 \
             WHERE id = $1 RETURNING {ACCOUNT_COLUMNS}"
        );

        let result = sqlx::query(&query)
            .bind(account.id)
            .bind(&account.name)
            .bind(account_type_str)
            .bind(account.balance)
            .bind(&account.currency)
            .bind(account.is_active)
            .bind(account.updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_account(result))
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
