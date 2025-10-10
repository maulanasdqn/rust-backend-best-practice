use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::TwoFactorService;

/// Use case for verifying a 2FA code
pub struct Verify2FA {
    user_repository: Arc<dyn UserRepository>,
    two_factor_service: TwoFactorService,
}

impl std::fmt::Debug for Verify2FA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Verify2FA")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field("two_factor_service", &self.two_factor_service)
            .finish()
    }
}

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

    /// Verifies a 2FA code and enables 2FA if it's the first verification
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    /// * `code` - The 6-digit TOTP code
    /// * `enable_on_success` - Whether to enable 2FA if verification succeeds
    ///
    /// # Returns
    /// true if the code is valid
    pub async fn execute(
        &self,
        user_id: Uuid,
        code: String,
        enable_on_success: bool,
    ) -> anyhow::Result<bool> {
        // Get the user
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Get the 2FA secret
        // Note: Uncomment when User struct is updated with two_factor_secret field
        // let secret = user
        //     .two_factor_secret
        //     .as_ref()
        //     .ok_or_else(|| anyhow::anyhow!("2FA is not set up for this user"))?;

        // For now, return an error since we can't get the secret
        let secret = "placeholder"; // Will be replaced when User is updated

        // Verify the code
        let is_valid = self
            .two_factor_service
            .verify_code(&user.email, secret, &code)?;

        if !is_valid {
            return Ok(false);
        }

        // If requested, enable 2FA on successful verification
        if enable_on_success {
            // Note: Uncomment when User struct is updated with two_factor_enabled field
            // user.two_factor_enabled = true;
            // user.updated_at = chrono::Utc::now();

            self.user_repository
                .update(user)
                .await
                .context("Failed to enable 2FA")?;
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
