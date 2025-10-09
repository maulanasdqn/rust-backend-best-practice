use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Transaction, TransactionRepository};

pub struct UpdateTransaction {
    repository: Arc<dyn TransactionRepository>,
}

impl std::fmt::Debug for UpdateTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateTransaction").finish()
    }
}

impl UpdateTransaction {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        category: Option<String>,
        description: Option<String>,
    ) -> Result<Transaction, AppError> {
        let mut transaction = self
            .repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Transaction not found".to_string()))?;

        transaction.update_details(category, description);

        self.repository
            .update(transaction)
            .await
            .map_err(AppError::from)
    }
}
