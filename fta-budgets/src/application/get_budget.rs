use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Budget, BudgetRepository};

pub struct GetBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl std::fmt::Debug for GetBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetBudget").finish()
    }
}

impl GetBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Budget, AppError> {
        self.repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Budget not found".to_string()))
    }
}
