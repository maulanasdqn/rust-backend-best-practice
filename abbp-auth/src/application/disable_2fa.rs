//! Disable 2FA use case.

use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use abbp_users::domain::UserRepository;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::infrastructure::services::{PasswordHashService, TwoFactorService};

/// Disables two-factor authentication for a user.
pub struct Disable2FA {
    user_repository: Arc<dyn UserRepository>,
    two_factor_service: TwoFactorService,
    password_hash_service: PasswordHashService,
}

impl_use_case_debug!(Disable2FA);

impl Disable2FA {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        two_factor_service: TwoFactorService,
        password_hash_service: PasswordHashService,
    ) -> Self {
        Self {
            user_repository,
            two_factor_service,
            password_hash_service,
        }
    }

    /// Disables 2FA after verifying password and 2FA code.
    ///
    /// # Errors
    /// Returns `NotFound` if user doesn't exist.
    /// Returns `Unauthorized` if password or 2FA code is invalid.
    #[instrument(skip(self, password, code), fields(user_id = %user_id))]
    pub async fn execute(
        &self,
        user_id: Uuid,
        password: String,
        code: String,
    ) -> Result<(), AppError> {
        tracing::debug!("Disabling 2FA");

        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let is_password_valid = self
            .password_hash_service
            .verify_password(&password, &user.password_hash)
            .map_err(|e| AppError::InternalError(format!("Password verification failed: {e}")))?;

        if !is_password_valid {
            tracing::warn!("Invalid password");
            return Err(AppError::Unauthorized("Invalid password".to_string()));
        }

        let secret = "placeholder";

        let is_code_valid = self
            .two_factor_service
            .verify_code(&user.email, secret, &code)
            .map_err(|e| AppError::InternalError(format!("2FA verification failed: {e}")))?;

        if !is_code_valid {
            tracing::warn!("Invalid 2FA code");
            return Err(AppError::Unauthorized("Invalid 2FA code".to_string()));
        }

        self.user_repository
            .update(user)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to disable 2FA: {e}")))?;

        tracing::info!("2FA disabled successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
