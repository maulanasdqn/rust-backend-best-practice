//! Get account use case.

use crate::domain::{Account, AccountRepository};
use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Retrieves a single account by ID.
pub struct GetAccount {
    repository: Arc<dyn AccountRepository>,
}

impl_use_case_debug!(GetAccount);

impl GetAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    /// Retrieves an account by its unique identifier.
    ///
    /// # Errors
    /// Returns `NotFound` if the account doesn't exist.
    #[instrument(skip(self), fields(account_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<Account, AppError> {
        self.repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))
    }
}
