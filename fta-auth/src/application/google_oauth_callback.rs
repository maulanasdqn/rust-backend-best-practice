use anyhow::Context;
use fta_users::domain::{User, UserRepository};
use std::sync::Arc;

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

pub struct GoogleOAuthCallback {
    user_repository: Arc<dyn UserRepository>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    google_oauth_service: GoogleOAuthService,
    jwt_service: JwtService,
    password_hash_service: PasswordHashService,
}

impl std::fmt::Debug for GoogleOAuthCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoogleOAuthCallback")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field(
                "refresh_token_repository",
                &"Arc<dyn RefreshTokenRepository>",
            )
            .field("google_oauth_service", &self.google_oauth_service)
            .field("jwt_service", &"JwtService")
            .field("password_hash_service", &self.password_hash_service)
            .finish()
    }
}

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

    pub async fn execute(&self, code: String) -> anyhow::Result<GoogleOAuthCallbackResult> {
        let (email, name, _google_id) = self
            .google_oauth_service
            .get_user_info(&code)
            .await
            .context("Failed to get user info from Google")?;

        let (user, is_new_user) = match self.user_repository.find_by_email(&email).await? {
            Some(existing_user) => {
                let user = existing_user;

                let updated_user = self
                    .user_repository
                    .update(user)
                    .await
                    .context("Failed to update user")?;

                (updated_user, false)
            },
            None => {
                let random_password = uuid::Uuid::new_v4().to_string();
                let password_hash = self.password_hash_service.hash_password(&random_password)?;

                let name_parts: Vec<&str> = name.split_whitespace().collect();
                let (first_name, last_name) = match name_parts.len() {
                    0 => (None, None),
                    1 => (Some(name_parts[0].to_string()), None),
                    _ => {
                        let first = name_parts[0].to_string();
                        let last = name_parts[1..].join(" ");
                        (Some(first), Some(last))
                    },
                };

                let user = User::new(email.clone(), password_hash, first_name, last_name);

                let created_user = self
                    .user_repository
                    .create(user)
                    .await
                    .context("Failed to create user")?;

                (created_user, true)
            },
        };

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

        Ok(GoogleOAuthCallbackResult {
            access_token,
            refresh_token: refresh_token_str,
            is_new_user,
        })
    }
}

#[cfg(test)]
mod tests {}
