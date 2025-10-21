use anyhow::Context;
use fta_users::domain::UserRepository;
use rand::Rng;
use std::sync::Arc;

use crate::{
    domain::{PasswordResetToken, PasswordResetTokenRepository},
    infrastructure::services::EmailService,
};

pub struct RequestPasswordReset {
    user_repository: Arc<dyn UserRepository>,
    password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    email_service: EmailService,
    base_url: String,
}

impl std::fmt::Debug for RequestPasswordReset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestPasswordReset")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field(
                "password_reset_token_repository",
                &"Arc<dyn PasswordResetTokenRepository>",
            )
            .field("email_service", &self.email_service)
            .field("base_url", &self.base_url)
            .finish()
    }
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

    pub async fn execute(&self, email: String) -> anyhow::Result<()> {
        let Some(user) = self.user_repository.find_by_email(&email).await? else {
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
            .context("Failed to create password reset token")?;

        let user_name = user.full_name().unwrap_or_else(|| email.clone());
        self.email_service
            .send_password_reset(&email, &user_name, &token, &self.base_url)
            .await
            .context("Failed to send password reset email")?;

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
