//! Repository trait for transaction persistence operations.

use async_trait::async_trait;
use fta_errors::AppError;
use uuid::Uuid;

use super::Transaction;

/// Repository trait defining persistence operations for transactions.
///
/// Implementations handle the actual database interactions while
/// the domain layer remains agnostic to storage details.
#[async_trait]
pub trait TransactionRepository: Send + Sync {
    /// Creates a new transaction in the database.
    async fn create(&self, transaction: Transaction) -> Result<Transaction, AppError>;

    /// Finds a transaction by its unique identifier.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Transaction>, AppError>;

    /// Retrieves a paginated list of transactions with filtering and sorting.
    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::TransactionFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>, AppError>;

    /// Counts all transactions matching the given filters.
    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::TransactionFilters,
    ) -> Result<i64, AppError>;

    /// Updates an existing transaction.
    async fn update(&self, transaction: Transaction) -> Result<Transaction, AppError>;

    /// Deletes a transaction by its unique identifier.
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}
