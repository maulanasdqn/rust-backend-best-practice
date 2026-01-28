//! User registration use case.

use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use fta_users::domain::{User, UserRepository};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    domain::{EmailVerification, EmailVerificationRepository},
    infrastructure::services::{EmailService, OtpService, PasswordHashService},
};

/// Registers a new user and sends verification email.
pub struct RegisterUser {
    user_repository: Arc<dyn UserRepository>,
    email_verification_repository: Arc<dyn EmailVerificationRepository>,
    password_hash_service: PasswordHashService,
    otp_service: OtpService,
    email_service: EmailService,
}

impl_use_case_debug!(RegisterUser);

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

    /// Registers a new user account.
    ///
    /// # Errors
    /// Returns `Conflict` if user with this email already exists.
    #[instrument(skip(self, password), fields(email = %email))]
    pub async fn execute(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<Uuid, AppError> {
        tracing::debug!("Registering new user");

        if self.user_repository.find_by_email(&email).await?.is_some() {
            tracing::warn!("Registration attempted with existing email");
            return Err(AppError::Conflict("User with this email already exists".to_string()));
        }

        let password_hash = self
            .password_hash_service
            .hash_password(&password)
            .map_err(|e| AppError::InternalError(format!("Failed to hash password: {e}")))?;

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
            .map_err(|e| AppError::InternalError(format!("Failed to create user: {e}")))?;

        let otp_code = self.otp_service.generate_code();

        let verification = EmailVerification::new(created_user.id, otp_code.clone(), 10);
        self.email_verification_repository
            .create(verification)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to create email verification: {e}")))?;

        let user_name = created_user.full_name().unwrap_or_else(|| email.clone());
        self.email_service
            .send_verification_otp(&email, &user_name, &otp_code)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to send verification email: {e}")))?;

        tracing::info!(user_id = %created_user.id, "User registered successfully");

        Ok(created_user.id)
    }
}

#[cfg(test)]
mod tests {}
