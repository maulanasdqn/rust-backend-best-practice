//! Refresh access token use case.

use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use tracing::instrument;

use crate::{domain::RefreshTokenRepository, infrastructure::services::JwtService};

/// Refreshes an access token using a valid refresh token.
pub struct RefreshAccessToken {
    user_repository: Arc<dyn UserRepository>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    jwt_service: JwtService,
}

impl_use_case_debug!(RefreshAccessToken);

impl RefreshAccessToken {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        refresh_token_repository: Arc<dyn RefreshTokenRepository>,
        jwt_service: JwtService,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
            jwt_service,
        }
    }

    /// Generates a new access token from a refresh token.
    ///
    /// # Errors
    /// Returns `Unauthorized` if the refresh token is invalid or expired.
    #[instrument(skip(self, refresh_token))]
    pub async fn execute(&self, refresh_token: String) -> Result<String, AppError> {
        tracing::debug!("Refreshing access token");

        let claims = self
            .jwt_service
            .verify_token(&refresh_token)
            .map_err(|_| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

        if !claims.is_refresh_token() {
            return Err(AppError::Unauthorized("Token is not a refresh token".to_string()));
        }

        let stored_token = self
            .refresh_token_repository
            .find_by_token(&refresh_token)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Refresh token not found or revoked".to_string()))?;

        if stored_token.is_expired() {
            let _ = self.refresh_token_repository.delete(&stored_token.id).await;
            return Err(AppError::Unauthorized("Refresh token has expired".to_string()));
        }

        let user = self
            .user_repository
            .find_by_id(&claims.sub)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        let access_token = self
            .jwt_service
            .generate_access_token(user.id, user.email)
            .map_err(|e| AppError::InternalError(format!("Failed to generate access token: {e}")))?;

        tracing::info!(user_id = %claims.sub, "Access token refreshed");

        Ok(access_token)
    }
}

#[cfg(test)]
mod tests {}
