use anyhow::Context;
use fta_users::domain::{User, UserRepository};
use std::sync::Arc;

use crate::{
    domain::{RefreshToken, RefreshTokenRepository},
    infrastructure::services::{GoogleOAuthService, JwtService, PasswordHashService},
};

/// Response from Google `OAuth` callback use case
#[derive(Debug)]
pub struct GoogleOAuthCallbackResult {
    pub access_token: String,
    pub refresh_token: String,
    pub is_new_user: bool,
}

/// Use case for handling Google `OAuth` callback
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

    /// Handles the `OAuth` callback and logs in or registers the user
    ///
    /// # Arguments
    /// * `code` - The authorization code from Google
    ///
    /// # Returns
    /// `GoogleOAuthCallbackResult` with tokens and user status
    pub async fn execute(&self, code: String) -> anyhow::Result<GoogleOAuthCallbackResult> {
        // Exchange code for user info
        let (email, name, _google_id) = self
            .google_oauth_service
            .get_user_info(&code)
            .await
            .context("Failed to get user info from Google")?;

        // Check if user exists
        let (user, is_new_user) = match self.user_repository.find_by_email(&email).await? {
            Some(existing_user) => {
                // User exists - verify it's the same Google account
                // Note: Uncomment when User struct is updated with oauth fields
                // if let Some(existing_oauth_id) = &existing_user.oauth_provider_id {
                //     if existing_oauth_id != &google_id {
                //         return Err(anyhow::anyhow!(
                //             "This email is already associated with a different account"
                //         ));
                //     }
                // }

                // Update last login
                let user = existing_user;
                // Note: Uncomment when User struct is updated
                // user.last_login_at = Some(Utc::now());
                // user.updated_at = Utc::now();

                let updated_user = self
                    .user_repository
                    .update(user)
                    .await
                    .context("Failed to update user")?;

                (updated_user, false)
            },
            None => {
                // Create new user
                // Generate a random password hash (user won't use it for OAuth)
                let random_password = uuid::Uuid::new_v4().to_string();
                let password_hash = self.password_hash_service.hash_password(&random_password)?;

                // Parse name into first_name and last_name
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

                // Set OAuth fields
                // Note: Uncomment when User struct is updated with oauth fields
                // user.oauth_provider = Some(OAuthProvider::Google);
                // user.oauth_provider_id = Some(google_id);
                // user.email_verified = true; // Google verifies emails
                // user.last_login_at = Some(Utc::now());

                let created_user = self
                    .user_repository
                    .create(user)
                    .await
                    .context("Failed to create user")?;

                (created_user, true)
            },
        };

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

        Ok(GoogleOAuthCallbackResult {
            access_token,
            refresh_token: refresh_token_str,
            is_new_user,
        })
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
