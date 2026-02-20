//! Update budget use case.

use crate::domain::{Budget, BudgetRepository};
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Updates an existing budget.
pub struct UpdateBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl_use_case_debug!(UpdateBudget);

impl UpdateBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    /// Updates a budget's category and/or amount.
    ///
    /// # Errors
    /// Returns `NotFound` if the budget doesn't exist.
    #[instrument(skip(self), fields(budget_id = %id))]
    pub async fn execute(
        &self,
        id: Uuid,
        category: Option<String>,
        amount: Option<i64>,
    ) -> Result<Budget, AppError> {
        let mut budget = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Budget not found".to_string()))?;

        if let Some(new_category) = category {
            budget.category = new_category;
            budget.updated_at = chrono::Utc::now();
        }

        if let Some(new_amount) = amount {
            budget.update_amount(new_amount);
        }

        let updated = self.repository.update(budget).await?;
        tracing::info!("Budget updated successfully");
        Ok(updated)
    }
}
