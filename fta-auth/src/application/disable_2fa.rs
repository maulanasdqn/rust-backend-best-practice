use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::{PasswordHashService, TwoFactorService};

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

    pub async fn execute(
        &self,
        user_id: Uuid,
        password: String,
        code: String,
    ) -> anyhow::Result<()> {
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let is_password_valid = self
            .password_hash_service
            .verify_password(&password, &user.password_hash)?;

        if !is_password_valid {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let secret = "placeholder";

        let is_code_valid = self
            .two_factor_service
            .verify_code(&user.email, secret, &code)?;

        if !is_code_valid {
            return Err(anyhow::anyhow!("Invalid 2FA code"));
        }

        self.user_repository
            .update(user)
            .await
            .context("Failed to disable 2FA")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
