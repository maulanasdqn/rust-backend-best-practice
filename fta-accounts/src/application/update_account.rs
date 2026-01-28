//! Update account use case.

use crate::domain::{Account, AccountRepository};
use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Updates an existing account.
pub struct UpdateAccount {
    repository: Arc<dyn AccountRepository>,
}

impl_use_case_debug!(UpdateAccount);

impl UpdateAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    /// Updates an account's name.
    ///
    /// # Errors
    /// Returns `NotFound` if the account doesn't exist.
    #[instrument(skip(self), fields(account_id = %id))]
    pub async fn execute(&self, id: Uuid, name: Option<String>) -> Result<Account, AppError> {
        let mut account = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

        if let Some(new_name) = name {
            account.rename(new_name);
        }

        let updated = self.repository.update(account).await?;
        tracing::info!("Account updated successfully");
        Ok(updated)
    }
}
