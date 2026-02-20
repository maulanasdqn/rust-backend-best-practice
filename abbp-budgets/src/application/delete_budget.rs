//! Delete budget use case.

use crate::domain::BudgetRepository;
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Deletes a budget.
pub struct DeleteBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl_use_case_debug!(DeleteBudget);

impl DeleteBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    /// Permanently deletes a budget.
    ///
    /// # Errors
    /// Returns `NotFound` if the budget doesn't exist.
    #[instrument(skip(self), fields(budget_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Budget not found".to_string()))?;

        self.repository.delete(&id).await?;
        tracing::info!("Budget deleted successfully");
        Ok(())
    }
}
