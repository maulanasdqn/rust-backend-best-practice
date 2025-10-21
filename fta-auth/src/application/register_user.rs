use anyhow::Context;
use fta_users::domain::{User, UserRepository};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{EmailVerification, EmailVerificationRepository},
    infrastructure::services::{EmailService, OtpService, PasswordHashService},
};

pub struct RegisterUser {
    user_repository: Arc<dyn UserRepository>,
    email_verification_repository: Arc<dyn EmailVerificationRepository>,
    password_hash_service: PasswordHashService,
    otp_service: OtpService,
    email_service: EmailService,
}

impl std::fmt::Debug for RegisterUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterUser")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field(
                "email_verification_repository",
                &"Arc<dyn EmailVerificationRepository>",
            )
            .field("password_hash_service", &self.password_hash_service)
            .field("otp_service", &self.otp_service)
            .field("email_service", &self.email_service)
            .finish()
    }
}

impl RegisterUser {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        email_verification_repository: Arc<dyn EmailVerificationRepository>,
        password_hash_service: PasswordHashService,
        otp_service: OtpService,
        email_service: EmailService,
    ) -> Self {
        Self {
            user_repository,
            email_verification_repository,
            password_hash_service,
            otp_service,
            email_service,
        }
    }

    pub async fn execute(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> anyhow::Result<Uuid> {
        if let Some(_existing_user) = self.user_repository.find_by_email(&email).await? {
            return Err(anyhow::anyhow!("User with this email already exists"));
        }

        let password_hash = self
            .password_hash_service
            .hash_password(&password)
            .context("Failed to hash password")?;

        let user = User::new(
            email.clone(),
            password_hash,
            first_name.clone(),
            last_name.clone(),
        );
        let created_user = self
            .user_repository
            .create(user)
            .await
            .context("Failed to create user")?;

        let otp_code = self.otp_service.generate_code();

        let verification = EmailVerification::new(created_user.id, otp_code.clone(), 10);
        self.email_verification_repository
            .create(verification)
            .await
            .context("Failed to create email verification")?;

        let user_name = created_user.full_name().unwrap_or_else(|| email.clone());
        self.email_service
            .send_verification_otp(&email, &user_name, &otp_code)
            .await
            .context("Failed to send verification email")?;

        Ok(created_user.id)
    }
}

#[cfg(test)]
mod tests {}
