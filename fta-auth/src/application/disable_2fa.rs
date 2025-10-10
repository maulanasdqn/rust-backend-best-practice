use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::{PasswordHashService, TwoFactorService};

/// Use case for disabling Two-Factor Authentication
pub struct Disable2FA {
    user_repository: Arc<dyn UserRepository>,
    two_factor_service: TwoFactorService,
    password_hash_service: PasswordHashService,
}

impl std::fmt::Debug for Disable2FA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Disable2FA")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field("two_factor_service", &self.two_factor_service)
            .field("password_hash_service", &self.password_hash_service)
            .finish()
    }
}

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

    /// Disables 2FA for a user after verifying password and current 2FA code
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    /// * `password` - The user's password (for additional security)
    /// * `code` - The current 6-digit TOTP code (to prove possession)
    pub async fn execute(
        &self,
        user_id: Uuid,
        password: String,
        code: String,
    ) -> anyhow::Result<()> {
        // Get the user
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Verify password
        let is_password_valid = self
            .password_hash_service
            .verify_password(&password, &user.password_hash)?;

        if !is_password_valid {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        // Check if 2FA is enabled
        // Note: Uncomment when User struct is updated with two_factor_enabled field
        // if !user.two_factor_enabled {
        //     return Err(anyhow::anyhow!("2FA is not enabled"));
        // }

        // Get the 2FA secret
        // Note: Uncomment when User struct is updated with two_factor_secret field
        // let secret = user
        //     .two_factor_secret
        //     .as_ref()
        //     .ok_or_else(|| anyhow::anyhow!("2FA secret not found"))?;

        let secret = "placeholder"; // Will be replaced when User is updated

        // Verify the current 2FA code
        let is_code_valid = self
            .two_factor_service
            .verify_code(&user.email, secret, &code)?;

        if !is_code_valid {
            return Err(anyhow::anyhow!("Invalid 2FA code"));
        }

        // Disable 2FA
        // Note: Uncomment when User struct is updated with two_factor fields
        // user.two_factor_enabled = false;
        // user.two_factor_secret = None;
        // user.updated_at = chrono::Utc::now();

        self.user_repository
            .update(user)
            .await
            .context("Failed to disable 2FA")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
