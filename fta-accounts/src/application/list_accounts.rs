use fta_errors::AppError;
use fta_types::PaginationQuery;
use std::sync::Arc;

use crate::domain::{Account, AccountRepository};
use crate::infrastructure::http::filters::AccountFilters;

pub struct ListAccounts {
    repository: Arc<dyn AccountRepository>,
}

impl std::fmt::Debug for ListAccounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListAccounts").finish()
    }
}

impl ListAccounts {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        filters: &AccountFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<Account>, i64), AppError> {
        let total = self
            .repository
            .count_all(filters)
            .await
            .map_err(AppError::from)?;
        let accounts = self
            .repository
            .find_all(
                filters,
                sort_by,
                sort_order,
                i64::from(pagination.limit()),
                i64::from(pagination.offset()),
            )
            .await
            .map_err(AppError::from)?;
        Ok((accounts, total))
    }
}
