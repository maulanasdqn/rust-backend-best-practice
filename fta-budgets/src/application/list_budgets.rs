use fta_errors::AppError;
use fta_types::PaginationQuery;
use std::sync::Arc;

use crate::domain::{Budget, BudgetRepository};
use crate::infrastructure::http::filters::BudgetFilters;

pub struct ListBudgets {
    repository: Arc<dyn BudgetRepository>,
}

impl std::fmt::Debug for ListBudgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListBudgets").finish()
    }
}

impl ListBudgets {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        filters: &BudgetFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<Budget>, i64), AppError> {
        let total = self
            .repository
            .count_all(filters)
            .await
            .map_err(AppError::from)?;
        let budgets = self
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
        Ok((budgets, total))
    }
}
