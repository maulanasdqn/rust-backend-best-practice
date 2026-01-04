use anyhow::Context;
use fta_users::domain::{User, UserRepository};
use std::sync::Arc;

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

    pub async fn execute(&self, email: String, password: String) -> anyhow::Result<LoginResult> {
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

        let is_valid = self
            .password_hash_service
            .verify_password(&password, &user.password_hash)?;

        if !is_valid {
            return Err(anyhow::anyhow!("Invalid email or password"));
        }

        let requires_2fa = false;

        self.user_repository
            .update(user.clone())
            .await
            .context("Failed to update user last login")?;

        let access_token = self
            .jwt_service
            .generate_access_token(user.id, user.email.clone())?;

        let (refresh_token_str, expires_at) = self
            .jwt_service
            .generate_refresh_token(user.id, user.email.clone())?;

        let refresh_token = RefreshToken::new(user.id, refresh_token_str.clone(), expires_at);
        self.refresh_token_repository
            .create(refresh_token)
            .await
            .context("Failed to store refresh token")?;

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
