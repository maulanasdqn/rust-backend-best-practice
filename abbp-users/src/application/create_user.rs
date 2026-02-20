//! Create user use case.

use crate::domain::{User, UserRepository};
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;

/// Creates a new user account.
pub struct CreateUser {
    repository: Arc<dyn UserRepository>,
}

impl_use_case_debug!(CreateUser);

impl CreateUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    /// Creates a new user with the given credentials.
    ///
    /// # Errors
    /// Returns `Conflict` if a user with the same email already exists.
    #[instrument(skip(self, password_hash), fields(email = %email))]
    pub async fn execute(
        &self,
        email: String,
        password_hash: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, AppError> {
        if self.repository.find_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict(
                "User with this email already exists".to_string(),
            ));
        }

        let user = User::new(email, password_hash, first_name, last_name);
        let created = self.repository.create(user).await?;
        tracing::info!(user_id = %created.id, "User created successfully");
        Ok(created)
    }
}
