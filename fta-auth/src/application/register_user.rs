use anyhow::Context;
use fta_users::domain::{User, UserRepository};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{EmailVerification, EmailVerificationRepository},
    infrastructure::services::{EmailService, OtpService, PasswordHashService},
};

/// Use case for registering a new user
pub struct RegisterUser {
    user_repository: Arc<dyn UserRepository>,
    email_verification_repository: Arc<dyn EmailVerificationRepository>,
    password_hash_service: PasswordHashService,
    otp_service: OtpService,
    email_service: EmailService,
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

    /// Registers a new user and sends verification email
    ///
    /// # Arguments
    /// * `email` - User's email address
    /// * `password` - User's plain text password
    /// * `first_name` - User's first name (optional)
    /// * `last_name` - User's last name (optional)
    ///
    /// # Returns
    /// The created user ID
    pub async fn execute(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> anyhow::Result<Uuid> {
        // Check if user already exists
        if let Some(_existing_user) = self.user_repository.find_by_email(&email).await? {
            return Err(anyhow::anyhow!("User with this email already exists"));
        }

        // Hash the password
        let password_hash = self
            .password_hash_service
            .hash_password(&password)
            .context("Failed to hash password")?;

        // Create the user
        let user = User::new(email.clone(), password_hash, first_name.clone(), last_name.clone());
        let created_user = self
            .user_repository
            .create(user)
            .await
            .context("Failed to create user")?;

        // Generate OTP code
        let otp_code = self.otp_service.generate_code();

        // Create email verification record
        let verification = EmailVerification::new(created_user.id, otp_code.clone(), 10); // 10 minutes validity
        self.email_verification_repository
            .create(verification)
            .await
            .context("Failed to create email verification")?;

        // Send verification email
        let user_name = created_user.full_name().unwrap_or_else(|| email.clone());
        self.email_service
            .send_verification_otp(&email, &user_name, &otp_code)
            .await
            .context("Failed to send verification email")?;

        Ok(created_user.id)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
    // Requires mocking repositories and services
}
