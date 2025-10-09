use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use fta_database::DbPool;
use sqlx::Row;
use uuid::Uuid;

use crate::domain::{Transaction, TransactionRepository, TransactionType};

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

        let result = sqlx::query(
      r"
      INSERT INTO transactions (id, account_id, transaction_type, amount, category, description, transaction_date, created_at, updated_at)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
      RETURNING id, account_id, transaction_type, amount, category, description, transaction_date, created_at, updated_at
      ",
    )
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

        let transaction_type: TransactionType =
            serde_json::from_str(result.get("transaction_type"))?;

        Ok(Transaction {
            id: result.get("id"),
            account_id: result.get("account_id"),
            transaction_type,
            amount: result.get("amount"),
            category: result.get("category"),
            description: result.get("description"),
            transaction_date: result.get("transaction_date"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Transaction>> {
        let result = sqlx::query(
      r"
      SELECT id, account_id, transaction_type, amount, category, description, transaction_date, created_at, updated_at
      FROM transactions
      WHERE id = $1
      ",
    )
    .bind(id)
    .fetch_optional(&self.pool)
    .await?;

        Ok(result.map(|row| {
            let transaction_type: TransactionType =
                serde_json::from_str(row.get("transaction_type"))
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
        }))
    }

    async fn find_by_account_id(
        &self,
        account_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>> {
        let results = sqlx::query(
      r"
      SELECT id, account_id, transaction_type, amount, category, description, transaction_date, created_at, updated_at
      FROM transactions
      WHERE account_id = $1
      ORDER BY transaction_date DESC
      LIMIT $2 OFFSET $3
      ",
    )
    .bind(account_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&self.pool)
    .await?;

        Ok(results
            .into_iter()
            .map(|row| {
                let transaction_type: TransactionType =
                    serde_json::from_str(row.get("transaction_type"))
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
            })
            .collect())
    }

    async fn count_by_account_id(&self, account_id: &Uuid) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            r"
      SELECT COUNT(*) FROM transactions WHERE account_id = $1
      ",
        )
        .bind(account_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    async fn find_by_date_range(
        &self,
        account_id: &Uuid,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Result<Vec<Transaction>> {
        let results = sqlx::query(
      r"
      SELECT id, account_id, transaction_type, amount, category, description, transaction_date, created_at, updated_at
      FROM transactions
      WHERE account_id = $1 AND transaction_date BETWEEN $2 AND $3
      ORDER BY transaction_date DESC
      ",
    )
    .bind(account_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(&self.pool)
    .await?;

        Ok(results
            .into_iter()
            .map(|row| {
                let transaction_type: TransactionType =
                    serde_json::from_str(row.get("transaction_type"))
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
            })
            .collect())
    }

    async fn update(&self, transaction: Transaction) -> Result<Transaction> {
        let transaction_type_str = serde_json::to_string(&transaction.transaction_type)?;

        let result = sqlx::query(
      r"
      UPDATE transactions
      SET transaction_type = $2, amount = $3, category = $4, description = $5, transaction_date = $6, updated_at = $7
      WHERE id = $1
      RETURNING id, account_id, transaction_type, amount, category, description, transaction_date, created_at, updated_at
      ",
    )
    .bind(transaction.id)
    .bind(transaction_type_str)
    .bind(transaction.amount)
    .bind(&transaction.category)
    .bind(&transaction.description)
    .bind(transaction.transaction_date)
    .bind(transaction.updated_at)
    .fetch_one(&self.pool)
    .await?;

        let transaction_type: TransactionType =
            serde_json::from_str(result.get("transaction_type"))?;

        Ok(Transaction {
            id: result.get("id"),
            account_id: result.get("account_id"),
            transaction_type,
            amount: result.get("amount"),
            category: result.get("category"),
            description: result.get("description"),
            transaction_date: result.get("transaction_date"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM transactions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
