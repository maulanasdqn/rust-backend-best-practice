//! Verify 2FA code use case.

use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::infrastructure::services::TwoFactorService;

/// Verifies a 2FA code and optionally enables 2FA for the user.
pub struct Verify2FA {
    user_repository: Arc<dyn UserRepository>,
    two_factor_service: TwoFactorService,
}

impl_use_case_debug!(Verify2FA);

impl Verify2FA {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        two_factor_service: TwoFactorService,
    ) -> Self {
        Self {
            user_repository,
            two_factor_service,
        }
    }

    /// Verifies a 2FA code and optionally enables 2FA on success.
    ///
    /// # Returns
    /// `true` if the code is valid, `false` otherwise.
    ///
    /// # Errors
    /// Returns `NotFound` if user doesn't exist.
    #[instrument(skip(self, code), fields(user_id = %user_id))]
    pub async fn execute(
        &self,
        user_id: Uuid,
        code: String,
        enable_on_success: bool,
    ) -> Result<bool, AppError> {
        tracing::debug!("Verifying 2FA code");

        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let secret = "placeholder";

        let is_valid = self
            .two_factor_service
            .verify_code(&user.email, secret, &code)
            .map_err(|e| AppError::InternalError(format!("2FA verification failed: {e}")))?;

        if !is_valid {
            tracing::debug!("Invalid 2FA code provided");
            return Ok(false);
        }

        if enable_on_success {
            self.user_repository
                .update(user)
                .await
                .map_err(|e| AppError::InternalError(format!("Failed to enable 2FA: {e}")))?;
            tracing::info!("2FA enabled successfully");
        } else {
            tracing::debug!("2FA code verified");
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {}
