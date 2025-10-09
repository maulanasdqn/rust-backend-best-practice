use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;

use crate::{
    domain::RefreshTokenRepository,
    infrastructure::services::JwtService,
};

/// Use case for refreshing an access token
pub struct RefreshAccessToken {
    user_repository: Arc<dyn UserRepository>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    jwt_service: JwtService,
}

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

    /// Generates a new access token from a refresh token
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token string
    ///
    /// # Returns
    /// A new access token
    pub async fn execute(&self, refresh_token: String) -> anyhow::Result<String> {
        // Verify the refresh token JWT
        let claims = self
            .jwt_service
            .verify_token(&refresh_token)
            .context("Invalid or expired refresh token")?;

        // Ensure it's a refresh token
        if !claims.is_refresh_token() {
            return Err(anyhow::anyhow!("Token is not a refresh token"));
        }

        // Find the refresh token in database
        let stored_token = self
            .refresh_token_repository
            .find_by_token(&refresh_token)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Refresh token not found or revoked"))?;

        // Check if token is expired
        if stored_token.is_expired() {
            // Clean up expired token
            let _ = self.refresh_token_repository.delete(&stored_token.id).await;
            return Err(anyhow::anyhow!("Refresh token has expired"));
        }

        // Get user to ensure they still exist
        let user = self
            .user_repository
            .find_by_id(&claims.sub)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Generate new access token
        let access_token = self
            .jwt_service
            .generate_access_token(user.id, user.email)?;

        Ok(access_token)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
