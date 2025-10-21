use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::TwoFactorService;

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

    pub async fn execute(
        &self,
        user_id: Uuid,
        code: String,
        enable_on_success: bool,
    ) -> anyhow::Result<bool> {
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let secret = "placeholder";

        let is_valid = self
            .two_factor_service
            .verify_code(&user.email, secret, &code)?;

        if !is_valid {
            return Ok(false);
        }

        if enable_on_success {
            self.user_repository
                .update(user)
                .await
                .context("Failed to enable 2FA")?;
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {}
