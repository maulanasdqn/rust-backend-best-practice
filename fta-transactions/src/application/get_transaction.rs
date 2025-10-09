use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Transaction, TransactionRepository};

pub struct GetTransaction {
    repository: Arc<dyn TransactionRepository>,
}

impl std::fmt::Debug for GetTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetTransaction").finish()
    }
}

impl GetTransaction {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Transaction, AppError> {
        self.repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Transaction not found".to_string()))
    }
}
