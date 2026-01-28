//! List accounts use case.

use crate::domain::{Account, AccountRepository};
use crate::infrastructure::http::filters::AccountFilters;
use fta_errors::AppError;
use fta_types::{impl_use_case_debug, PaginationQuery};
use std::sync::Arc;
use tracing::instrument;

/// Lists accounts with filtering and pagination.
pub struct ListAccounts {
    repository: Arc<dyn AccountRepository>,
}

impl_use_case_debug!(ListAccounts);

impl ListAccounts {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    /// Retrieves a paginated list of accounts.
    ///
    /// # Returns
    /// A tuple of (accounts, total_count) for pagination.
    #[instrument(skip(self, filters, pagination))]
    pub async fn execute(
        &self,
        filters: &AccountFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<Account>, i64), AppError> {
        let total = self.repository.count_all(filters).await?;
        let accounts = self
            .repository
            .find_all(
                filters,
                sort_by,
                sort_order,
                i64::from(pagination.limit()),
                i64::from(pagination.offset()),
            )
            .await?;
        tracing::debug!(count = accounts.len(), total, "Listed accounts");
        Ok((accounts, total))
    }
}
