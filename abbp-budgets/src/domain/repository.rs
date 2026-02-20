//! Repository trait for budget persistence operations.

use async_trait::async_trait;
use abbp_errors::AppError;
use uuid::Uuid;

use super::Budget;

/// Repository trait defining persistence operations for budgets.
///
/// Implementations handle the actual database interactions while
/// the domain layer remains agnostic to storage details.
#[async_trait]
pub trait BudgetRepository: Send + Sync {
    /// Creates a new budget in the database.
    async fn create(&self, budget: Budget) -> Result<Budget, AppError>;

    /// Finds a budget by its unique identifier.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Budget>, AppError>;

    /// Retrieves a paginated list of budgets with filtering and sorting.
    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::BudgetFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Budget>, AppError>;

    /// Counts all budgets matching the given filters.
    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::BudgetFilters,
    ) -> Result<i64, AppError>;

    /// Updates an existing budget.
    async fn update(&self, budget: Budget) -> Result<Budget, AppError>;

    /// Deletes a budget by its unique identifier.
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}
