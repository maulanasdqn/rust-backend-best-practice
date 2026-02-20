//! Delete user use case.

use crate::domain::UserRepository;
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Deletes a user account.
pub struct DeleteUser {
    repository: Arc<dyn UserRepository>,
}

impl_use_case_debug!(DeleteUser);

impl DeleteUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    /// Permanently deletes a user account.
    ///
    /// # Errors
    /// Returns `NotFound` if the user doesn't exist.
    #[instrument(skip(self), fields(user_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        self.repository.delete(&id).await?;
        tracing::info!("User deleted successfully");
        Ok(())
    }
}
