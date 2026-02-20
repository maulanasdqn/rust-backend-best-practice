//! Email verification use case.

use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use abbp_users::domain::UserRepository;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::EmailVerificationRepository;

/// Verifies a user's email address using an OTP code.
pub struct VerifyEmail {
    user_repository: Arc<dyn UserRepository>,
    email_verification_repository: Arc<dyn EmailVerificationRepository>,
}

impl_use_case_debug!(VerifyEmail);

impl VerifyEmail {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_verification_repository: Arc<dyn EmailVerificationRepository>,
    ) -> Self {
        Self {
            user_repository,
            email_verification_repository,
        }
    }

    /// Verifies the user's email with the provided OTP code.
    ///
    /// # Errors
    /// Returns `NotFound` if user doesn't exist.
    /// Returns `BadRequest` if OTP code is invalid or expired.
    #[instrument(skip(self, otp_code), fields(user_id = %user_id))]
    pub async fn execute(&self, user_id: Uuid, otp_code: String) -> Result<(), AppError> {
        tracing::debug!("Verifying email");

        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let verification = self
            .email_verification_repository
            .find_by_user_id(&user_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("No verification code found for this user".to_string()))?;

        if !verification.is_valid(&otp_code) {
            tracing::warn!("Invalid or expired OTP code");
            return Err(AppError::BadRequest("Invalid or expired OTP code".to_string()));
        }

        self.user_repository
            .update(user)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to update user: {e}")))?;

        self.email_verification_repository
            .delete(&verification.id)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to delete verification record: {e}")))?;

        tracing::info!("Email verified successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
