//! PostgreSQL implementation of the transaction repository.

use async_trait::async_trait;
use abbp_database::{entities::transactions, sea_orm, DbPool};
use abbp_errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::domain::{Transaction, TransactionRepository, TransactionType};
use crate::infrastructure::http::filters::TransactionFilters;

fn model_to_transaction(model: transactions::Model) -> Transaction {
    let transaction_type: TransactionType =
        serde_json::from_str(&model.transaction_type).unwrap_or(TransactionType::Expense);

    Transaction {
        id: model.id,
        account_id: model.account_id,
        transaction_type,
        amount: model.amount,
        category: model.category,
        description: model.description,
        transaction_date: model.transaction_date,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

/// Converts a database error to an application error.
fn db_err(e: impl std::fmt::Display) -> AppError {
    AppError::InternalError(format!("Database error: {e}"))
}

#[derive(Clone, Debug)]
pub struct PostgresTransactionRepository {
    pool: DbPool,
}

impl PostgresTransactionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionRepository for PostgresTransactionRepository {
    async fn create(&self, transaction: Transaction) -> Result<Transaction, AppError> {
        let transaction_type_str =
            serde_json::to_string(&transaction.transaction_type).map_err(db_err)?;

        let active_model = transactions::ActiveModel {
            id: Set(transaction.id),
            account_id: Set(transaction.account_id),
            transaction_type: Set(transaction_type_str),
            amount: Set(transaction.amount),
            category: Set(transaction.category.clone()),
            description: Set(transaction.description.clone()),
            transaction_date: Set(transaction.transaction_date),
            created_at: Set(transaction.created_at),
            updated_at: Set(transaction.updated_at),
        };

        let result = active_model.insert(&self.pool).await.map_err(db_err)?;
        Ok(model_to_transaction(result))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Transaction>, AppError> {
        let result = transactions::Entity::find_by_id(*id)
            .one(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(result.map(model_to_transaction))
    }

    async fn find_all(
        &self,
        filters: &TransactionFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>, AppError> {
        let mut query = transactions::Entity::find();

        // Apply filters
        if let Some(account_id) = filters.account_id {
            query = query.filter(transactions::Column::AccountId.eq(account_id));
        }
        if let Some(ref transaction_type) = filters.transaction_type {
            let type_str = serde_json::to_string(transaction_type).unwrap_or_default();
            query = query.filter(transactions::Column::TransactionType.eq(type_str));
        }
        if let Some(ref category) = filters.category {
            query = query.filter(transactions::Column::Category.eq(category));
        }
        if let Some(min_amount) = filters.min_amount {
            query = query.filter(transactions::Column::Amount.gte(min_amount));
        }
        if let Some(max_amount) = filters.max_amount {
            query = query.filter(transactions::Column::Amount.lte(max_amount));
        }
        if let Some(start_date) = filters.start_date {
            query = query.filter(transactions::Column::TransactionDate.gte(start_date));
        }
        if let Some(end_date) = filters.end_date {
            query = query.filter(transactions::Column::TransactionDate.lte(end_date));
        }
        if let Some(ref search) = filters.search {
            query = query.filter(transactions::Column::Description.contains(search));
        }

        // Apply sorting
        let order = if sort_order.to_lowercase() == "asc" {
            sea_orm::Order::Asc
        } else {
            sea_orm::Order::Desc
        };

        query = match sort_by {
            Some("amount") => query.order_by(transactions::Column::Amount, order),
            Some("category") => query.order_by(transactions::Column::Category, order),
            Some("transaction_date") | None => {
                query.order_by(transactions::Column::TransactionDate, order)
            }
            Some("created_at") => query.order_by(transactions::Column::CreatedAt, order),
            Some(_) => query.order_by(transactions::Column::TransactionDate, order),
        };

        let results = query
            .paginate(&self.pool, limit as u64)
            .fetch_page((offset / limit.max(1)) as u64)
            .await
            .map_err(db_err)?;

        Ok(results.into_iter().map(model_to_transaction).collect())
    }

    async fn count_all(&self, filters: &TransactionFilters) -> Result<i64, AppError> {
        let mut query = transactions::Entity::find();

        // Apply filters
        if let Some(account_id) = filters.account_id {
            query = query.filter(transactions::Column::AccountId.eq(account_id));
        }
        if let Some(ref transaction_type) = filters.transaction_type {
            let type_str = serde_json::to_string(transaction_type).unwrap_or_default();
            query = query.filter(transactions::Column::TransactionType.eq(type_str));
        }
        if let Some(ref category) = filters.category {
            query = query.filter(transactions::Column::Category.eq(category));
        }
        if let Some(min_amount) = filters.min_amount {
            query = query.filter(transactions::Column::Amount.gte(min_amount));
        }
        if let Some(max_amount) = filters.max_amount {
            query = query.filter(transactions::Column::Amount.lte(max_amount));
        }
        if let Some(start_date) = filters.start_date {
            query = query.filter(transactions::Column::TransactionDate.gte(start_date));
        }
        if let Some(end_date) = filters.end_date {
            query = query.filter(transactions::Column::TransactionDate.lte(end_date));
        }
        if let Some(ref search) = filters.search {
            query = query.filter(transactions::Column::Description.contains(search));
        }

        let count = query.count(&self.pool).await.map_err(db_err)?;
        Ok(count as i64)
    }

    async fn update(&self, transaction: Transaction) -> Result<Transaction, AppError> {
        let transaction_type_str =
            serde_json::to_string(&transaction.transaction_type).map_err(db_err)?;

        let active_model = transactions::ActiveModel {
            id: Set(transaction.id),
            account_id: Set(transaction.account_id),
            transaction_type: Set(transaction_type_str),
            amount: Set(transaction.amount),
            category: Set(transaction.category.clone()),
            description: Set(transaction.description.clone()),
            transaction_date: Set(transaction.transaction_date),
            created_at: Set(transaction.created_at),
            updated_at: Set(transaction.updated_at),
        };

        let result = active_model.update(&self.pool).await.map_err(db_err)?;
        Ok(model_to_transaction(result))
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        transactions::Entity::delete_by_id(*id)
            .exec(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }
}
