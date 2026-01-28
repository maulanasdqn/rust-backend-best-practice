//! Get user use case.

use crate::domain::{User, UserRepository};
use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Retrieves a single user by ID.
pub struct GetUser {
    repository: Arc<dyn UserRepository>,
}

impl_use_case_debug!(GetUser);

impl GetUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    /// Retrieves a user by their unique identifier.
    ///
    /// # Errors
    /// Returns `NotFound` if the user doesn't exist.
    #[instrument(skip(self), fields(user_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<User, AppError> {
        self.repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }
}
