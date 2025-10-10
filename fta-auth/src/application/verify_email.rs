use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::EmailVerificationRepository;

/// Use case for verifying a user's email address
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

    /// Verifies a user's email with the provided OTP code
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    /// * `otp_code` - The 6-digit OTP code
    ///
    /// # Returns
    /// Success if verification is valid
    pub async fn execute(&self, user_id: Uuid, otp_code: String) -> anyhow::Result<()> {
        // Find the user
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Note: This assumes email_verified field will be added to User
        // For now, this is a placeholder - will need to update once User struct is extended
        // if user.email_verified {
        //     return Err(anyhow::anyhow!("Email already verified"));
        // }

        // Find the verification record
        let verification = self
            .email_verification_repository
            .find_by_user_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No verification code found for this user"))?;

        // Validate the OTP code
        if !verification.is_valid(&otp_code) {
            return Err(anyhow::anyhow!("Invalid or expired OTP code"));
        }

        // Mark user as verified
        // user.email_verified = true;  // Will be uncommented when User struct is updated
        // user.updated_at = chrono::Utc::now();

        self.user_repository
            .update(user)
            .await
            .context("Failed to update user")?;

        // Delete the verification record
        self.email_verification_repository
            .delete(&verification.id)
            .await
            .context("Failed to delete verification record")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
