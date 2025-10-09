use fta_errors::AppError;
use std::sync::Arc;

use crate::domain::{User, UserRepository};

pub struct CreateUser {
    repository: Arc<dyn UserRepository>,
}

impl std::fmt::Debug for CreateUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateUser").finish()
    }
}

impl CreateUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        email: String,
        password_hash: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, AppError> {
        // Check if user already exists
        if let Some(_existing) = self
            .repository
            .find_by_email(&email)
            .await
            .map_err(AppError::from)?
        {
            return Err(AppError::Conflict(
                "User with this email already exists".to_string(),
            ));
        }

        let user = User::new(email, password_hash, first_name, last_name);

        self.repository.create(user).await.map_err(AppError::from)
    }
}
