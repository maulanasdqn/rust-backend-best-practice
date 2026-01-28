//! List users use case.

use crate::domain::{User, UserRepository};
use crate::infrastructure::http::filters::UserFilters;
use fta_errors::AppError;
use fta_types::{impl_use_case_debug, PaginationQuery};
use std::sync::Arc;
use tracing::instrument;

/// Lists users with filtering and pagination.
pub struct ListUsers {
    repository: Arc<dyn UserRepository>,
}

impl_use_case_debug!(ListUsers);

impl ListUsers {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    /// Retrieves a paginated list of users.
    ///
    /// # Returns
    /// A tuple of (users, total_count) for pagination.
    #[instrument(skip(self, filters, pagination))]
    pub async fn execute(
        &self,
        filters: &UserFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<User>, i64), AppError> {
        let total = self.repository.count_all(filters).await?;
        let users = self
            .repository
            .find_all(
                filters,
                sort_by,
                sort_order,
                i64::from(pagination.limit()),
                i64::from(pagination.offset()),
            )
            .await?;
        tracing::debug!(count = users.len(), total, "Listed users");
        Ok((users, total))
    }
}
