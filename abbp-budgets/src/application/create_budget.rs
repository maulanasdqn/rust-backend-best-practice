//! Create budget use case.

use chrono::{DateTime, Utc};
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::{Budget, BudgetPeriod, BudgetRepository};

/// Creates a new budget.
pub struct CreateBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl_use_case_debug!(CreateBudget);

impl CreateBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    /// Creates a new budget for a user.
    #[instrument(skip(self), fields(user_id = %user_id, category = %category))]
    pub async fn execute(
        &self,
        user_id: Uuid,
        category: String,
        amount: i64,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
    ) -> Result<Budget, AppError> {
        tracing::debug!("Creating new budget");
        let budget = Budget::new(user_id, category, amount, period, start_date);
        let result = self.repository.create(budget).await?;
        tracing::info!(budget_id = %result.id, "Budget created successfully");
        Ok(result)
    }
}
