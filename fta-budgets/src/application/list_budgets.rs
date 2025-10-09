use fta_errors::AppError;
use paginator_rs::PaginationParams;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Budget, BudgetRepository};

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
        user_id: Uuid,
        params: &PaginationParams,
    ) -> Result<(Vec<Budget>, i64), AppError> {
        let total = self
            .repository
            .count_by_user_id(&user_id)
            .await
            .map_err(AppError::from)?;
        let budgets = self
            .repository
            .find_by_user_id(
                &user_id,
                i64::from(params.limit()),
                i64::from(params.offset()),
            )
            .await
            .map_err(AppError::from)?;
        Ok((budgets, total))
    }
}
