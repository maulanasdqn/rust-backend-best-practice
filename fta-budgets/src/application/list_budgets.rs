//! List budgets use case.

use crate::domain::{Budget, BudgetRepository};
use crate::infrastructure::http::filters::BudgetFilters;
use fta_errors::AppError;
use fta_types::{impl_use_case_debug, PaginationQuery};
use std::sync::Arc;
use tracing::instrument;

/// Lists budgets with filtering and pagination.
pub struct ListBudgets {
    repository: Arc<dyn BudgetRepository>,
}

impl_use_case_debug!(ListBudgets);

impl ListBudgets {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    /// Retrieves a paginated list of budgets.
    ///
    /// # Returns
    /// A tuple of (budgets, total_count) for pagination.
    #[instrument(skip(self, filters, pagination))]
    pub async fn execute(
        &self,
        filters: &BudgetFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<Budget>, i64), AppError> {
        let total = self.repository.count_all(filters).await?;
        let budgets = self
            .repository
            .find_all(
                filters,
                sort_by,
                sort_order,
                i64::from(pagination.limit()),
                i64::from(pagination.offset()),
            )
            .await?;
        tracing::debug!(count = budgets.len(), total, "Listed budgets");
        Ok((budgets, total))
    }
}
