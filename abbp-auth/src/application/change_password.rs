//! Change password use case.

use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use abbp_users::domain::UserRepository;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::infrastructure::services::PasswordHashService;

/// Changes a user's password after verifying the current one.
pub struct ChangePassword {
    user_repository: Arc<dyn UserRepository>,
    password_hash_service: PasswordHashService,
}

impl_use_case_debug!(ChangePassword);

impl ChangePassword {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hash_service: PasswordHashService,
    ) -> Self {
        Self {
            user_repository,
            password_hash_service,
        }
    }

    /// Changes the user's password.
    ///
    /// # Errors
    /// Returns `NotFound` if user doesn't exist.
    /// Returns `Unauthorized` if current password is incorrect.
    #[instrument(skip(self, current_password, new_password), fields(user_id = %user_id))]
    pub async fn execute(
        &self,
        user_id: Uuid,
        current_password: String,
        new_password: String,
    ) -> Result<(), AppError> {
        tracing::debug!("Changing password");

        let mut user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let is_valid = self
            .password_hash_service
            .verify_password(&current_password, &user.password_hash)
            .map_err(|e| AppError::InternalError(format!("Password verification failed: {e}")))?;

        if !is_valid {
            tracing::warn!("Invalid current password");
            return Err(AppError::Unauthorized("Current password is incorrect".to_string()));
        }

        let new_password_hash = self
            .password_hash_service
            .hash_password(&new_password)
            .map_err(|e| AppError::InternalError(format!("Failed to hash password: {e}")))?;

        user.change_password(new_password_hash);

        self.user_repository
            .update(user)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to update user password: {e}")))?;

        tracing::info!("Password changed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
