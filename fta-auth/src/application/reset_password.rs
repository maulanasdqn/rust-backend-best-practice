use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;

use crate::{
    domain::PasswordResetTokenRepository,
    infrastructure::services::PasswordHashService,
};

/// Use case for resetting a password with a reset token
pub struct ResetPassword {
    user_repository: Arc<dyn UserRepository>,
    password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    password_hash_service: PasswordHashService,
}

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

    /// Resets a user's password using a reset token
    ///
    /// # Arguments
    /// * `token` - The password reset token
    /// * `new_password` - The new password (plain text)
    pub async fn execute(&self, token: String, new_password: String) -> anyhow::Result<()> {
        // Find the reset token
        let mut reset_token = self
            .password_reset_token_repository
            .find_by_token(&token)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid or expired reset token"))?;

        // Validate the token
        if !reset_token.is_valid() {
            return Err(anyhow::anyhow!("Invalid or expired reset token"));
        }

        // Get the user
        let mut user = self
            .user_repository
            .find_by_id(&reset_token.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Hash the new password
        let new_password_hash = self
            .password_hash_service
            .hash_password(&new_password)
            .context("Failed to hash password")?;

        // Update the user's password
        user.change_password(new_password_hash);

        self.user_repository
            .update(user)
            .await
            .context("Failed to update user password")?;

        // Mark the token as used
        reset_token.mark_as_used();
        self.password_reset_token_repository
            .update(reset_token)
            .await
            .context("Failed to update reset token")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
