use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::BudgetRepository;

pub struct DeleteBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl std::fmt::Debug for DeleteBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteBudget").finish()
    }
}

impl DeleteBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Budget not found".to_string()))?;

        self.repository.delete(&id).await.map_err(AppError::from)
    }
}
