use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::TransactionRepository;

pub struct DeleteTransaction {
    repository: Arc<dyn TransactionRepository>,
}

impl std::fmt::Debug for DeleteTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteTransaction").finish()
    }
}

impl DeleteTransaction {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Transaction not found".to_string()))?;

        self.repository.delete(&id).await.map_err(AppError::from)
    }
}
