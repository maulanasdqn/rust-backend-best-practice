use anyhow::Result;
use async_trait::async_trait;
use fta_database::DbPool;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::domain::{Transaction, TransactionRepository, TransactionType};
use crate::infrastructure::http::filters::TransactionFilters;

const TRANSACTION_COLUMNS: &str =
    "id, account_id, transaction_type, amount, category, description, \
     transaction_date, created_at, updated_at";

fn row_to_transaction(row: PgRow) -> Transaction {
    let transaction_type: TransactionType = serde_json::from_str(row.get("transaction_type"))
        .unwrap_or(TransactionType::Expense);

    Transaction {
        id: row.get("id"),
        account_id: row.get("account_id"),
        transaction_type,
        amount: row.get("amount"),
        category: row.get("category"),
        description: row.get("description"),
        transaction_date: row.get("transaction_date"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn apply_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Postgres>,
    filters: &'a TransactionFilters,
) {
    if let Some(account_id) = filters.account_id {
        query_builder.push(" AND account_id = ");
        query_builder.push_bind(account_id);
    }

    if let Some(ref transaction_type) = filters.transaction_type {
        let type_str = serde_json::to_string(transaction_type).unwrap_or_default();
        query_builder.push(" AND transaction_type = ");
        query_builder.push_bind(type_str);
    }

    if let Some(ref category) = filters.category {
        query_builder.push(" AND category = ");
        query_builder.push_bind(category);
    }

    if let Some(min_amount) = filters.min_amount {
        query_builder.push(" AND amount >= ");
        query_builder.push_bind(min_amount);
    }

    if let Some(max_amount) = filters.max_amount {
        query_builder.push(" AND amount <= ");
        query_builder.push_bind(max_amount);
    }

    if let Some(start_date) = filters.start_date {
        query_builder.push(" AND transaction_date >= ");
        query_builder.push_bind(start_date);
    }

    if let Some(end_date) = filters.end_date {
        query_builder.push(" AND transaction_date <= ");
        query_builder.push_bind(end_date);
    }

    if let Some(ref search) = filters.search {
        query_builder.push(" AND description ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
    }
}

#[derive(Clone, Debug)]
pub struct PostgresTransactionRepository {
    pool: DbPool,
}

impl PostgresTransactionRepository {
    pub const fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionRepository for PostgresTransactionRepository {
    async fn create(&self, transaction: Transaction) -> Result<Transaction> {
        let transaction_type_str = serde_json::to_string(&transaction.transaction_type)?;
        let query = format!(
            "INSERT INTO transactions ({TRANSACTION_COLUMNS}) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
             RETURNING {TRANSACTION_COLUMNS}"
        );

        let result = sqlx::query(&query)
            .bind(transaction.id)
            .bind(transaction.account_id)
            .bind(transaction_type_str)
            .bind(transaction.amount)
            .bind(&transaction.category)
            .bind(&transaction.description)
            .bind(transaction.transaction_date)
            .bind(transaction.created_at)
            .bind(transaction.updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_transaction(result))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Transaction>> {
        let query = format!("SELECT {TRANSACTION_COLUMNS} FROM transactions WHERE id = $1");

        let result = sqlx::query(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(row_to_transaction))
    }

    async fn find_all(
        &self,
        filters: &TransactionFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>> {
        let mut query_builder = sqlx::QueryBuilder::new(format!(
            "SELECT {TRANSACTION_COLUMNS} FROM transactions WHERE 1=1"
        ));

        apply_filters(&mut query_builder, filters);

        if let Some(sort_field) = sort_by {
            query_builder.push(format!(" ORDER BY {sort_field} {sort_order}"));
        } else {
            query_builder.push(" ORDER BY transaction_date DESC");
        }

        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        let results = query_builder.build().fetch_all(&self.pool).await?;

        Ok(results.into_iter().map(row_to_transaction).collect())
    }

    async fn count_all(&self, filters: &TransactionFilters) -> Result<i64> {
        let mut query_builder =
            sqlx::QueryBuilder::new("SELECT COUNT(*) FROM transactions WHERE 1=1");

        apply_filters(&mut query_builder, filters);

        let result: (i64,) = query_builder.build_query_as().fetch_one(&self.pool).await?;

        Ok(result.0)
    }

    async fn update(&self, transaction: Transaction) -> Result<Transaction> {
        let transaction_type_str = serde_json::to_string(&transaction.transaction_type)?;
        let query = format!(
            "UPDATE transactions SET transaction_type = $2, amount = $3, category = $4, \
             description = $5, transaction_date = $6, updated_at = $7 \
             WHERE id = $1 RETURNING {TRANSACTION_COLUMNS}"
        );

        let result = sqlx::query(&query)
            .bind(transaction.id)
            .bind(transaction_type_str)
            .bind(transaction.amount)
            .bind(&transaction.category)
            .bind(&transaction.description)
            .bind(transaction.transaction_date)
            .bind(transaction.updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_transaction(result))
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM transactions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
