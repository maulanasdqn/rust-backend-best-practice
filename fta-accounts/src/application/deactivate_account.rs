//! Deactivate account use case.

use crate::domain::{Account, AccountRepository};
use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Deactivates an account without deleting it.
pub struct DeactivateAccount {
    repository: Arc<dyn AccountRepository>,
}

impl_use_case_debug!(DeactivateAccount);

impl DeactivateAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    /// Deactivates an account, making it inactive.
    ///
    /// # Errors
    /// Returns `NotFound` if the account doesn't exist.
    #[instrument(skip(self), fields(account_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<Account, AppError> {
        let mut account = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

        account.deactivate();

        let updated = self.repository.update(account).await?;
        tracing::info!("Account deactivated successfully");
        Ok(updated)
    }
}
