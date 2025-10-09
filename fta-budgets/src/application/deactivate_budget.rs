use chrono::{DateTime, Utc};
use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Budget, BudgetRepository};

pub struct DeactivateBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl std::fmt::Debug for DeactivateBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeactivateBudget").finish()
    }
}

impl DeactivateBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid, end_date: DateTime<Utc>) -> Result<Budget, AppError> {
        let mut budget = self
            .repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Budget not found".to_string()))?;

        budget.deactivate(end_date);

        self.repository.update(budget).await.map_err(AppError::from)
    }
}
