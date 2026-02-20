//! Request password reset use case.

use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use abbp_users::domain::UserRepository;
use rand::Rng;
use std::sync::Arc;
use tracing::instrument;

use crate::{
    domain::{PasswordResetToken, PasswordResetTokenRepository},
    infrastructure::services::EmailService,
};

/// Initiates a password reset by sending a reset email.
pub struct RequestPasswordReset {
    user_repository: Arc<dyn UserRepository>,
    password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    email_service: EmailService,
    base_url: String,
}

impl_use_case_debug!(RequestPasswordReset);

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

    /// Sends a password reset email if the user exists.
    ///
    /// Note: Always returns Ok(()) to prevent email enumeration attacks.
    #[instrument(skip(self), fields(email = %email))]
    pub async fn execute(&self, email: String) -> Result<(), AppError> {
        tracing::debug!("Processing password reset request");

        let Some(user) = self.user_repository.find_by_email(&email).await? else {
            tracing::debug!("No user found for email, silently succeeding");
            return Ok(());
        };

        let _ = self
            .password_reset_token_repository
            .delete_by_user_id(&user.id)
            .await;

        let token = Self::generate_secure_token();

        let reset_token = PasswordResetToken::new(user.id, token.clone(), 60);
        self.password_reset_token_repository
            .create(reset_token)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to create password reset token: {e}")))?;

        let user_name = user.full_name().unwrap_or_else(|| email.clone());
        self.email_service
            .send_password_reset(&email, &user_name, &token, &self.base_url)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to send password reset email: {e}")))?;

        tracing::info!(user_id = %user.id, "Password reset email sent");

        Ok(())
    }

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

        assert_eq!(token1.len(), 64);
        assert_eq!(token2.len(), 64);

        assert_ne!(token1, token2);

        assert!(token1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(token2.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
