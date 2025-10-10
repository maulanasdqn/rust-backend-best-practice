use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;

use crate::{
    domain::{RefreshToken, RefreshTokenRepository},
    infrastructure::services::{JwtService, PasswordHashService},
};

/// Response from successful login use case
#[derive(Debug)]
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
    pub requires_2fa: bool,
}

/// Use case for user login
pub struct Login {
    user_repository: Arc<dyn UserRepository>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    password_hash_service: PasswordHashService,
    jwt_service: JwtService,
}

impl std::fmt::Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Login")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field(
                "refresh_token_repository",
                &"Arc<dyn RefreshTokenRepository>",
            )
            .field("password_hash_service", &self.password_hash_service)
            .field("jwt_service", &"JwtService")
            .finish()
    }
}

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

    /// Authenticates a user and generates tokens
    ///
    /// # Arguments
    /// * `email` - User's email address
    /// * `password` - User's plain text password
    ///
    /// # Returns
    /// `LoginResult` containing access token, refresh token, and 2FA status
    pub async fn execute(&self, email: String, password: String) -> anyhow::Result<LoginResult> {
        // Find the user
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

        // Verify password
        let is_valid = self
            .password_hash_service
            .verify_password(&password, &user.password_hash)?;

        if !is_valid {
            return Err(anyhow::anyhow!("Invalid email or password"));
        }

        // Check if email is verified
        // Note: Uncomment when User struct is updated with email_verified field
        // if !user.email_verified {
        //     return Err(anyhow::anyhow!("Email not verified. Please verify your email first."));
        // }

        // Check if 2FA is enabled
        // Note: Uncomment when User struct is updated with two_factor_enabled field
        let requires_2fa = false; // user.two_factor_enabled;

        // Update last login time
        // Note: Uncomment when User struct is updated with last_login_at field
        // user.last_login_at = Some(Utc::now());
        // user.updated_at = Utc::now();

        self.user_repository
            .update(user.clone())
            .await
            .context("Failed to update user last login")?;

        // Generate JWT tokens
        let access_token = self
            .jwt_service
            .generate_access_token(user.id, user.email.clone())?;

        let (refresh_token_str, expires_at) = self
            .jwt_service
            .generate_refresh_token(user.id, user.email.clone())?;

        // Store refresh token in database
        let refresh_token = RefreshToken::new(user.id, refresh_token_str.clone(), expires_at);
        self.refresh_token_repository
            .create(refresh_token)
            .await
            .context("Failed to store refresh token")?;

        Ok(LoginResult {
            access_token,
            refresh_token: refresh_token_str,
            requires_2fa,
        })
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
