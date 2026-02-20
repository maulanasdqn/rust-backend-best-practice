//! User login use case.

use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use abbp_users::domain::{User, UserRepository};
use std::sync::Arc;
use tracing::instrument;

use crate::{
    domain::{RefreshToken, RefreshTokenRepository},
    infrastructure::services::{JwtService, PasswordHashService},
};

#[derive(Debug)]
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
    pub requires_2fa: bool,
    pub user: User,
}

/// Authenticates a user and issues JWT tokens.
pub struct Login {
    user_repository: Arc<dyn UserRepository>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    password_hash_service: PasswordHashService,
    jwt_service: JwtService,
}

impl_use_case_debug!(Login);

impl Login {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        refresh_token_repository: Arc<dyn RefreshTokenRepository>,
        password_hash_service: PasswordHashService,
        jwt_service: JwtService,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
            password_hash_service,
            jwt_service,
        }
    }

    /// Authenticates a user with email and password.
    ///
    /// # Errors
    /// Returns `Unauthorized` if credentials are invalid.
    #[instrument(skip(self, password), fields(email = %email))]
    pub async fn execute(&self, email: String, password: String) -> Result<LoginResult, AppError> {
        tracing::debug!("Attempting login");

        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        let is_valid = self
            .password_hash_service
            .verify_password(&password, &user.password_hash)
            .map_err(|e| AppError::InternalError(format!("Password verification failed: {e}")))?;

        if !is_valid {
            tracing::warn!("Invalid password attempt");
            return Err(AppError::Unauthorized("Invalid email or password".to_string()));
        }

        let requires_2fa = false;

        self.user_repository
            .update(user.clone())
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to update user last login: {e}")))?;

        let access_token = self
            .jwt_service
            .generate_access_token(user.id, user.email.clone())
            .map_err(|e| AppError::InternalError(format!("Failed to generate access token: {e}")))?;

        let (refresh_token_str, expires_at) = self
            .jwt_service
            .generate_refresh_token(user.id, user.email.clone())
            .map_err(|e| AppError::InternalError(format!("Failed to generate refresh token: {e}")))?;

        let refresh_token = RefreshToken::new(user.id, refresh_token_str.clone(), expires_at);
        self.refresh_token_repository
            .create(refresh_token)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to store refresh token: {e}")))?;

        tracing::info!(user_id = %user.id, "Login successful");

        Ok(LoginResult {
            access_token,
            refresh_token: refresh_token_str,
            requires_2fa,
            user,
        })
    }
}

#[cfg(test)]
mod tests {}
