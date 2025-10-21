use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;

use crate::{domain::PasswordResetTokenRepository, infrastructure::services::PasswordHashService};

pub struct ResetPassword {
    user_repository: Arc<dyn UserRepository>,
    password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    password_hash_service: PasswordHashService,
}

impl std::fmt::Debug for ResetPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResetPassword")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field(
                "password_reset_token_repository",
                &"Arc<dyn PasswordResetTokenRepository>",
            )
            .field("password_hash_service", &self.password_hash_service)
            .finish()
    }
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

    pub async fn execute(&self, token: String, new_password: String) -> anyhow::Result<()> {
        let mut reset_token = self
            .password_reset_token_repository
            .find_by_token(&token)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid or expired reset token"))?;

        if !reset_token.is_valid() {
            return Err(anyhow::anyhow!("Invalid or expired reset token"));
        }

        let mut user = self
            .user_repository
            .find_by_id(&reset_token.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let new_password_hash = self
            .password_hash_service
            .hash_password(&new_password)
            .context("Failed to hash password")?;

        user.change_password(new_password_hash);

        self.user_repository
            .update(user)
            .await
            .context("Failed to update user password")?;

        reset_token.mark_as_used();
        self.password_reset_token_repository
            .update(reset_token)
            .await
            .context("Failed to update reset token")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
