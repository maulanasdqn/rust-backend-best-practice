//! Repository trait for account persistence operations.

use async_trait::async_trait;
use abbp_errors::AppError;
use uuid::Uuid;

use super::Account;

/// Repository trait defining persistence operations for accounts.
///
/// Implementations handle the actual database interactions while
/// the domain layer remains agnostic to storage details.
#[async_trait]
pub trait AccountRepository: Send + Sync {
    /// Creates a new account in the database.
    async fn create(&self, account: Account) -> Result<Account, AppError>;

    /// Finds an account by its unique identifier.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Account>, AppError>;

    /// Retrieves a paginated list of accounts with filtering and sorting.
    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::AccountFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Account>, AppError>;

    /// Counts all accounts matching the given filters.
    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::AccountFilters,
    ) -> Result<i64, AppError>;

    /// Updates an existing account.
    async fn update(&self, account: Account) -> Result<Account, AppError>;

    /// Deletes an account by its unique identifier.
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}
