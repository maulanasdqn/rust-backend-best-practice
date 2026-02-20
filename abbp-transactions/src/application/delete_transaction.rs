//! Delete transaction use case.

use crate::domain::TransactionRepository;
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Deletes a transaction.
pub struct DeleteTransaction {
    repository: Arc<dyn TransactionRepository>,
}

impl_use_case_debug!(DeleteTransaction);

impl DeleteTransaction {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    /// Permanently deletes a transaction.
    ///
    /// # Errors
    /// Returns `NotFound` if the transaction doesn't exist.
    #[instrument(skip(self), fields(transaction_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Transaction not found".to_string()))?;

        self.repository.delete(&id).await?;
        tracing::info!("Transaction deleted successfully");
        Ok(())
    }
}
