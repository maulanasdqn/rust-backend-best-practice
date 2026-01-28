//! Google OAuth callback use case.

use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use fta_users::domain::{User, UserRepository};
use std::sync::Arc;
use tracing::instrument;

use crate::{
    domain::{RefreshToken, RefreshTokenRepository},
    infrastructure::services::{GoogleOAuthService, JwtService, PasswordHashService},
};

#[derive(Debug)]
pub struct GoogleOAuthCallbackResult {
    pub access_token: String,
    pub refresh_token: String,
    pub is_new_user: bool,
}

/// Handles the Google OAuth callback and authenticates/registers the user.
pub struct GoogleOAuthCallback {
    user_repository: Arc<dyn UserRepository>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    google_oauth_service: GoogleOAuthService,
    jwt_service: JwtService,
    password_hash_service: PasswordHashService,
}

impl_use_case_debug!(GoogleOAuthCallback);

impl GoogleOAuthCallback {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        refresh_token_repository: Arc<dyn RefreshTokenRepository>,
        google_oauth_service: GoogleOAuthService,
        jwt_service: JwtService,
        password_hash_service: PasswordHashService,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
            google_oauth_service,
            jwt_service,
            password_hash_service,
        }
    }

    /// Processes the OAuth callback, creating or updating the user.
    ///
    /// # Errors
    /// Returns `Unauthorized` if Google authentication fails.
    #[instrument(skip(self, code))]
    pub async fn execute(&self, code: String) -> Result<GoogleOAuthCallbackResult, AppError> {
        tracing::debug!("Processing Google OAuth callback");

        let (email, name, _google_id) = self
            .google_oauth_service
            .get_user_info(&code)
            .await
            .map_err(|e| AppError::Unauthorized(format!("Failed to get user info from Google: {e}")))?;

        let (user, is_new_user) = match self.user_repository.find_by_email(&email).await? {
            Some(existing_user) => {
                let user = existing_user;

                let updated_user = self
                    .user_repository
                    .update(user)
                    .await
                    .map_err(|e| AppError::InternalError(format!("Failed to update user: {e}")))?;

                tracing::info!(user_id = %updated_user.id, "Existing user authenticated via Google");
                (updated_user, false)
            }
            None => {
                let random_password = uuid::Uuid::new_v4().to_string();
                let password_hash = self
                    .password_hash_service
                    .hash_password(&random_password)
                    .map_err(|e| AppError::InternalError(format!("Failed to hash password: {e}")))?;

                let name_parts: Vec<&str> = name.split_whitespace().collect();
                let (first_name, last_name) = match name_parts.len() {
                    0 => (None, None),
                    1 => (Some(name_parts[0].to_string()), None),
                    _ => {
                        let first = name_parts[0].to_string();
                        let last = name_parts[1..].join(" ");
                        (Some(first), Some(last))
                    }
                };

                let user = User::new(email.clone(), password_hash, first_name, last_name);

                let created_user = self
                    .user_repository
                    .create(user)
                    .await
                    .map_err(|e| AppError::InternalError(format!("Failed to create user: {e}")))?;

                tracing::info!(user_id = %created_user.id, "New user created via Google OAuth");
                (created_user, true)
            }
        };

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

        Ok(GoogleOAuthCallbackResult {
            access_token,
            refresh_token: refresh_token_str,
            is_new_user,
        })
    }
}

#[cfg(test)]
mod tests {}
