//! Update transaction use case.

use crate::domain::{Transaction, TransactionRepository};
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Updates an existing transaction.
pub struct UpdateTransaction {
    repository: Arc<dyn TransactionRepository>,
}

impl_use_case_debug!(UpdateTransaction);

impl UpdateTransaction {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    /// Updates a transaction's category and description.
    ///
    /// # Errors
    /// Returns `NotFound` if the transaction doesn't exist.
    #[instrument(skip(self), fields(transaction_id = %id))]
    pub async fn execute(
        &self,
        id: Uuid,
        category: Option<String>,
        description: Option<String>,
    ) -> Result<Transaction, AppError> {
        let mut transaction = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Transaction not found".to_string()))?;

        transaction.update_details(category, description);

        let updated = self.repository.update(transaction).await?;
        tracing::info!("Transaction updated successfully");
        Ok(updated)
    }
}
