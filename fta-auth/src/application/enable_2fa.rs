use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::TwoFactorService;

/// Response from Enable2FA use case
pub struct Enable2FAResult {
    pub secret: String,
    pub qr_code_svg: String,
    pub provisioning_uri: String,
}

/// Use case for enabling Two-Factor Authentication
pub struct Enable2FA {
    user_repository: Arc<dyn UserRepository>,
    two_factor_service: TwoFactorService,
}

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

    /// Enables 2FA for a user and returns setup information
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    ///
    /// # Returns
    /// Enable2FAResult containing the secret, QR code, and provisioning URI
    pub async fn execute(&self, user_id: Uuid) -> anyhow::Result<Enable2FAResult> {
        // Get the user
        let mut user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Check if 2FA is already enabled
        // Note: Uncomment when User struct is updated with two_factor_enabled field
        // if user.two_factor_enabled {
        //     return Err(anyhow::anyhow!("2FA is already enabled"));
        // }

        // Generate TOTP secret
        let secret = self.two_factor_service.generate_secret();

        // Generate QR code
        let qr_code_svg = self
            .two_factor_service
            .generate_qr_code(&user.email, &secret)
            .context("Failed to generate QR code")?;

        // Get provisioning URI (for manual entry)
        let provisioning_uri = self
            .two_factor_service
            .get_provisioning_uri(&user.email, &secret)
            .context("Failed to generate provisioning URI")?;

        // Store the secret (but don't enable 2FA yet - user must verify first)
        // Note: Uncomment when User struct is updated with two_factor_secret field
        // user.two_factor_secret = Some(secret.clone());
        // user.two_factor_enabled = false; // Not enabled until verified
        // user.updated_at = chrono::Utc::now();

        self.user_repository
            .update(user)
            .await
            .context("Failed to update user")?;

        Ok(Enable2FAResult {
            secret,
            qr_code_svg,
            provisioning_uri,
        })
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
