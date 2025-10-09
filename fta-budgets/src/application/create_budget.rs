use chrono::{DateTime, Utc};
use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Budget, BudgetPeriod, BudgetRepository};

pub struct CreateBudget {
    repository: Arc<dyn BudgetRepository>,
}

impl std::fmt::Debug for CreateBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateBudget").finish()
    }
}

impl CreateBudget {
    pub fn new(repository: Arc<dyn BudgetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        category: String,
        amount: i64,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
    ) -> Result<Budget, AppError> {
        let budget = Budget::new(user_id, category, amount, period, start_date);
        self.repository.create(budget).await.map_err(AppError::from)
    }
}
