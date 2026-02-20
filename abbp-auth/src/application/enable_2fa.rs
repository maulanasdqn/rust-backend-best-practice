//! Enable 2FA use case.

use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use abbp_users::domain::UserRepository;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::infrastructure::services::TwoFactorService;

#[derive(Debug)]
pub struct Enable2FAResult {
    pub secret: String,
    pub qr_code_svg: String,
    pub provisioning_uri: String,
}

/// Enables two-factor authentication for a user.
pub struct Enable2FA {
    user_repository: Arc<dyn UserRepository>,
    two_factor_service: TwoFactorService,
}

impl_use_case_debug!(Enable2FA);

impl Enable2FA {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        two_factor_service: TwoFactorService,
    ) -> Self {
        Self {
            user_repository,
            two_factor_service,
        }
    }

    /// Generates 2FA setup data (secret, QR code, provisioning URI).
    ///
    /// # Errors
    /// Returns `NotFound` if user doesn't exist.
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn execute(&self, user_id: Uuid) -> Result<Enable2FAResult, AppError> {
        tracing::debug!("Enabling 2FA");

        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let secret = self.two_factor_service.generate_secret();

        let qr_code_svg = self
            .two_factor_service
            .generate_qr_code(&user.email, &secret)
            .map_err(|e| AppError::InternalError(format!("Failed to generate QR code: {e}")))?;

        let provisioning_uri = self
            .two_factor_service
            .get_provisioning_uri(&user.email, &secret)
            .map_err(|e| AppError::InternalError(format!("Failed to generate provisioning URI: {e}")))?;

        self.user_repository
            .update(user)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to update user: {e}")))?;

        tracing::info!("2FA setup data generated");

        Ok(Enable2FAResult {
            secret,
            qr_code_svg,
            provisioning_uri,
        })
    }
}

#[cfg(test)]
mod tests {}
