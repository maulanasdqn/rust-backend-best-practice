use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::EmailVerificationRepository;

pub struct VerifyEmail {
    user_repository: Arc<dyn UserRepository>,
    email_verification_repository: Arc<dyn EmailVerificationRepository>,
}

impl std::fmt::Debug for VerifyEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VerifyEmail")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field(
                "email_verification_repository",
                &"Arc<dyn EmailVerificationRepository>",
            )
            .finish()
    }
}

impl VerifyEmail {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_verification_repository: Arc<dyn EmailVerificationRepository>,
    ) -> Self {
        Self {
            user_repository,
            email_verification_repository,
        }
    }

    pub async fn execute(&self, user_id: Uuid, otp_code: String) -> anyhow::Result<()> {
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let verification = self
            .email_verification_repository
            .find_by_user_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No verification code found for this user"))?;

        if !verification.is_valid(&otp_code) {
            return Err(anyhow::anyhow!("Invalid or expired OTP code"));
        }

        self.user_repository
            .update(user)
            .await
            .context("Failed to update user")?;

        self.email_verification_repository
            .delete(&verification.id)
            .await
            .context("Failed to delete verification record")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
