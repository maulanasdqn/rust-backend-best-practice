use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::TwoFactorService;

#[derive(Debug)]
pub struct Enable2FAResult {
    pub secret: String,
    pub qr_code_svg: String,
    pub provisioning_uri: String,
}

pub struct Enable2FA {
    user_repository: Arc<dyn UserRepository>,
    two_factor_service: TwoFactorService,
}

impl std::fmt::Debug for Enable2FA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enable2FA")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field("two_factor_service", &self.two_factor_service)
            .finish()
    }
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

    pub async fn execute(&self, user_id: Uuid) -> anyhow::Result<Enable2FAResult> {
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let secret = self.two_factor_service.generate_secret();

        let qr_code_svg = self
            .two_factor_service
            .generate_qr_code(&user.email, &secret)
            .context("Failed to generate QR code")?;

        let provisioning_uri = self
            .two_factor_service
            .get_provisioning_uri(&user.email, &secret)
            .context("Failed to generate provisioning URI")?;

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
mod tests {}
