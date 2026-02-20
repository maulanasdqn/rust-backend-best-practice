//! Deactivate budget use case.

use chrono::{DateTime, Utc};
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::{Budget, BudgetRepository};

/// Deactivates a budget by setting an end date.
pub struct DeactivateBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl_use_case_debug!(DeactivateBudget);

impl DeactivateBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    /// Deactivates a budget by setting its end date.
    ///
    /// # Errors
    /// Returns `NotFound` if the budget doesn't exist.
    #[instrument(skip(self), fields(budget_id = %id))]
    pub async fn execute(&self, id: Uuid, end_date: DateTime<Utc>) -> Result<Budget, AppError> {
        let mut budget = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Budget not found".to_string()))?;

        budget.deactivate(end_date);

        let updated = self.repository.update(budget).await?;
        tracing::info!("Budget deactivated successfully");
        Ok(updated)
    }
}
