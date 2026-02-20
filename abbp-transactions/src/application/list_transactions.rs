//! List transactions use case.

use crate::domain::{Transaction, TransactionRepository};
use crate::infrastructure::http::filters::TransactionFilters;
use abbp_errors::AppError;
use abbp_types::{impl_use_case_debug, PaginationQuery};
use std::sync::Arc;
use tracing::instrument;

/// Lists transactions with filtering and pagination.
pub struct ListTransactions {
    repository: Arc<dyn TransactionRepository>,
}

impl_use_case_debug!(ListTransactions);

impl ListTransactions {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    /// Retrieves a paginated list of transactions.
    ///
    /// # Returns
    /// A tuple of (transactions, total_count) for pagination.
    #[instrument(skip(self, filters, pagination))]
    pub async fn execute(
        &self,
        filters: &TransactionFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<Transaction>, i64), AppError> {
        let total = self.repository.count_all(filters).await?;
        let transactions = self
            .repository
            .find_all(
                filters,
                sort_by,
                sort_order,
                i64::from(pagination.limit()),
                i64::from(pagination.offset()),
            )
            .await?;
        tracing::debug!(count = transactions.len(), total, "Listed transactions");
        Ok((transactions, total))
    }
}
