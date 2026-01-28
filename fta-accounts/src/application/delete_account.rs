//! Delete account use case.

use crate::domain::AccountRepository;
use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Deletes an account.
pub struct DeleteAccount {
    repository: Arc<dyn AccountRepository>,
}

impl_use_case_debug!(DeleteAccount);

impl DeleteAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    /// Permanently deletes an account.
    ///
    /// # Errors
    /// Returns `NotFound` if the account doesn't exist.
    #[instrument(skip(self), fields(account_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

        self.repository.delete(&id).await?;
        tracing::info!("Account deleted successfully");
        Ok(())
    }
}
