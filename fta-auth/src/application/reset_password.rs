//! Reset password use case.

use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use tracing::instrument;

use crate::{domain::PasswordResetTokenRepository, infrastructure::services::PasswordHashService};

/// Resets a user's password using a valid reset token.
pub struct ResetPassword {
    user_repository: Arc<dyn UserRepository>,
    password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    password_hash_service: PasswordHashService,
}

impl_use_case_debug!(ResetPassword);

impl ResetPassword {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
        password_hash_service: PasswordHashService,
    ) -> Self {
        Self {
            user_repository,
            password_reset_token_repository,
            password_hash_service,
        }
    }

    /// Resets the user's password using the provided reset token.
    ///
    /// # Errors
    /// Returns `BadRequest` if token is invalid or expired.
    /// Returns `NotFound` if user doesn't exist.
    #[instrument(skip(self, token, new_password))]
    pub async fn execute(&self, token: String, new_password: String) -> Result<(), AppError> {
        tracing::debug!("Resetting password");

        let mut reset_token = self
            .password_reset_token_repository
            .find_by_token(&token)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired reset token".to_string()))?;

        if !reset_token.is_valid() {
            tracing::warn!("Reset token is invalid or expired");
            return Err(AppError::BadRequest("Invalid or expired reset token".to_string()));
        }

        let mut user = self
            .user_repository
            .find_by_id(&reset_token.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let new_password_hash = self
            .password_hash_service
            .hash_password(&new_password)
            .map_err(|e| AppError::InternalError(format!("Failed to hash password: {e}")))?;

        user.change_password(new_password_hash);

        self.user_repository
            .update(user)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to update user password: {e}")))?;

        reset_token.mark_as_used();
        self.password_reset_token_repository
            .update(reset_token)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to update reset token: {e}")))?;

        tracing::info!("Password reset successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
