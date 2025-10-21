use fta_errors::AppError;
use fta_types::PaginationQuery;
use std::sync::Arc;

use crate::domain::{Transaction, TransactionRepository};
use crate::infrastructure::http::filters::TransactionFilters;

pub struct ListTransactions {
    repository: Arc<dyn TransactionRepository>,
}

impl std::fmt::Debug for ListTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListTransactions").finish()
    }
}

impl ListTransactions {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        filters: &TransactionFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<Transaction>, i64), AppError> {
        let total = self
            .repository
            .count_all(filters)
            .await
            .map_err(AppError::from)?;
        let transactions = self
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
        Ok((transactions, total))
    }
}
