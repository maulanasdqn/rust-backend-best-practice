use anyhow::Context;
use fta_users::domain::UserRepository;
use rand::Rng;
use std::sync::Arc;

use crate::{
    domain::{PasswordResetToken, PasswordResetTokenRepository},
    infrastructure::services::EmailService,
};

/// Use case for requesting a password reset
pub struct RequestPasswordReset {
    user_repository: Arc<dyn UserRepository>,
    password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    email_service: EmailService,
    base_url: String,
}

impl RequestPasswordReset {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
        email_service: EmailService,
        base_url: String,
    ) -> Self {
        Self {
            user_repository,
            password_reset_token_repository,
            email_service,
            base_url,
        }
    }

    /// Generates a reset token and sends a password reset email
    ///
    /// # Arguments
    /// * `email` - User's email address
    ///
    /// Note: Always returns Ok(()) even if user doesn't exist (security best practice)
    pub async fn execute(&self, email: String) -> anyhow::Result<()> {
        // Find the user
        let user = match self.user_repository.find_by_email(&email).await? {
            Some(user) => user,
            None => {
                // Don't reveal that user doesn't exist (security best practice)
                return Ok(());
            }
        };

        // Delete any existing reset tokens for this user
        let _ = self
            .password_reset_token_repository
            .delete_by_user_id(&user.id)
            .await;

        // Generate secure random token
        let token = Self::generate_secure_token();

        // Create password reset token (valid for 60 minutes)
        let reset_token = PasswordResetToken::new(user.id, token.clone(), 60);
        self.password_reset_token_repository
            .create(reset_token)
            .await
            .context("Failed to create password reset token")?;

        // Send password reset email
        let user_name = user.full_name().unwrap_or_else(|| email.clone());
        self.email_service
            .send_password_reset(&email, &user_name, &token, &self.base_url)
            .await
            .context("Failed to send password reset email")?;

        Ok(())
    }

    /// Generates a cryptographically secure random token
    fn generate_secure_token() -> String {
        let mut rng = rand::rng();
        let token_bytes: Vec<u8> = (0..32).map(|_| rng.random::<u8>()).collect();
        hex::encode(token_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secure_token() {
        let token1 = RequestPasswordReset::generate_secure_token();
        let token2 = RequestPasswordReset::generate_secure_token();

        // Tokens should be 64 characters (32 bytes as hex)
        assert_eq!(token1.len(), 64);
        assert_eq!(token2.len(), 64);

        // Tokens should be different
        assert_ne!(token1, token2);

        // Tokens should only contain hex characters
        assert!(token1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(token2.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
