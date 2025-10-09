use serde::Deserialize;

/// Service for Google OAuth authentication
#[derive(Clone)]
pub struct GoogleOAuthService {
    client_id: String,
    redirect_uri: String,
    client_secret: String,
    http_client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    id: String,
    email: String,
    verified_email: Option<bool>,
    name: Option<String>,
}

impl GoogleOAuthService {
    /// Creates a new GoogleOAuthService
    ///
    /// # Arguments
    /// * `client_id` - Google OAuth client ID
    /// * `client_secret` - Google OAuth client secret
    /// * `redirect_uri` - OAuth redirect URI (e.g., "http://localhost:8080/api/auth/google/callback")
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            redirect_uri,
            client_secret,
            http_client: reqwest::Client::new(),
        }
    }

    /// Generates the Google OAuth authorization URL
    ///
    /// This URL should be used to redirect the user to Google's consent screen
    ///
    /// # Returns
    /// The authorization URL as a string
    pub fn get_authorization_url(&self) -> String {
        // Build Google OAuth URL manually
        let scopes = "openid email profile";
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}",
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(scopes)
        )
    }

    /// Exchanges the authorization code for user information
    ///
    /// # Arguments
    /// * `code` - The authorization code received from Google's callback
    ///
    /// # Returns
    /// A tuple of (email, name, google_id)
    pub async fn get_user_info(&self, code: &str) -> anyhow::Result<(String, String, String)> {
        // Step 1: Exchange authorization code for access token
        let token_response = self
            .http_client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("code", code),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("redirect_uri", &self.redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to exchange code for token: {}", e))?;

        if !token_response.status().is_success() {
            let error_text = token_response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Token exchange failed: {}", error_text));
        }

        let token_data: GoogleTokenResponse = token_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse token response: {}", e))?;

        // Step 2: Use access token to get user info
        let user_info_response = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(&token_data.access_token)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch user info: {}", e))?;

        if !user_info_response.status().is_success() {
            let error_text = user_info_response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("User info fetch failed: {}", error_text));
        }

        let user_info: GoogleUserInfo = user_info_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse user info: {}", e))?;

        let email = user_info.email;
        let name = user_info.name.unwrap_or_else(|| "Google User".to_string());
        let google_id = user_info.id;

        Ok((email, name, google_id))
    }

    /// Validates that the user has verified their email with Google
    ///
    /// # Arguments
    /// * `code` - The authorization code received from Google's callback
    ///
    /// # Returns
    /// true if the email is verified, false otherwise
    pub async fn is_email_verified(&self, code: &str) -> anyhow::Result<bool> {
        // Exchange code for access token
        let token_response = self
            .http_client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("code", code),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("redirect_uri", &self.redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to exchange code for token: {}", e))?;

        let token_data: GoogleTokenResponse = token_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse token response: {}", e))?;

        // Fetch user info
        let user_info_response = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(&token_data.access_token)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch user info: {}", e))?;

        let user_info: GoogleUserInfo = user_info_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse user info: {}", e))?;

        Ok(user_info.verified_email.unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_authorization_url() {
        let service = GoogleOAuthService::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:8080/callback".to_string(),
        );

        let auth_url = service.get_authorization_url();

        // Verify the URL contains expected parameters
        assert!(auth_url.contains("client_id=test_client_id"));
        assert!(auth_url.contains("redirect_uri="));
        assert!(auth_url.contains("scope="));
        assert!(auth_url.contains("response_type=code"));
    }

    #[tokio::test]
    #[ignore] // Requires valid Google OAuth credentials and authorization code
    async fn test_get_user_info() {
        let service = GoogleOAuthService::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:8080/callback".to_string(),
        );

        // This test requires a valid authorization code from Google
        // In practice, you would mock this or use integration tests
        let result = service.get_user_info("invalid_code").await;

        assert!(result.is_err());
    }
}
