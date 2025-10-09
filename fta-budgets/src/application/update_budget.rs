use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Budget, BudgetRepository};

pub struct UpdateBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl std::fmt::Debug for UpdateBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateBudget").finish()
    }
}

impl UpdateBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        category: Option<String>,
        amount: Option<i64>,
    ) -> Result<Budget, AppError> {
        let mut budget = self
            .repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Budget not found".to_string()))?;

        if let Some(new_category) = category {
            budget.category = new_category;
            budget.updated_at = chrono::Utc::now();
        }

        if let Some(new_amount) = amount {
            budget.update_amount(new_amount);
        }

        self.repository.update(budget).await.map_err(AppError::from)
    }
}
