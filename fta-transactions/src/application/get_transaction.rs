//! Get transaction use case.

use crate::domain::{Transaction, TransactionRepository};
use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Retrieves a single transaction by ID.
pub struct GetTransaction {
    repository: Arc<dyn TransactionRepository>,
}

impl_use_case_debug!(GetTransaction);

impl GetTransaction {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    /// Retrieves a transaction by its unique identifier.
    ///
    /// # Errors
    /// Returns `NotFound` if the transaction doesn't exist.
    #[instrument(skip(self), fields(transaction_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<Transaction, AppError> {
        self.repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Transaction not found".to_string()))
    }
}
