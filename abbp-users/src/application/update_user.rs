//! Update user use case.

use crate::domain::{User, UserRepository};
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Updates an existing user's profile.
pub struct UpdateUser {
    repository: Arc<dyn UserRepository>,
}

impl_use_case_debug!(UpdateUser);

impl UpdateUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    /// Updates a user's profile information.
    ///
    /// # Errors
    /// Returns `NotFound` if the user doesn't exist.
    #[instrument(skip(self), fields(user_id = %id))]
    pub async fn execute(
        &self,
        id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, AppError> {
        let mut user = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        user.update_profile(first_name, last_name);

        let updated = self.repository.update(user).await?;
        tracing::info!("User profile updated successfully");
        Ok(updated)
    }
}
