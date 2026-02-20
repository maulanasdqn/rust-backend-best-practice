use serde::Deserialize;

#[derive(Clone, Debug)]
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
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            redirect_uri,
            client_secret,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn get_authorization_url(&self) -> String {
        let scopes = "openid email profile";
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}",
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(scopes)
        )
    }

    pub async fn get_user_info(&self, code: &str) -> anyhow::Result<(String, String, String)> {
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
            .map_err(|e| anyhow::anyhow!("Failed to exchange code for token: {e}"))?;

        if !token_response.status().is_success() {
            let error_text = token_response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Token exchange failed: {error_text}"));
        }

        let token_data: GoogleTokenResponse = token_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse token response: {e}"))?;

        let user_info_response = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(&token_data.access_token)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch user info: {e}"))?;

        if !user_info_response.status().is_success() {
            let error_text = user_info_response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("User info fetch failed: {error_text}"));
        }

        let user_info: GoogleUserInfo = user_info_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse user info: {e}"))?;

        let email = user_info.email;
        let name = user_info.name.unwrap_or_else(|| "Google User".to_string());
        let google_id = user_info.id;

        Ok((email, name, google_id))
    }

    pub async fn is_email_verified(&self, code: &str) -> anyhow::Result<bool> {
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
            .map_err(|e| anyhow::anyhow!("Failed to exchange code for token: {e}"))?;

        let token_data: GoogleTokenResponse = token_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse token response: {e}"))?;

        let user_info_response = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(&token_data.access_token)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch user info: {e}"))?;

        let user_info: GoogleUserInfo = user_info_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse user info: {e}"))?;

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

        assert!(auth_url.contains("client_id=test_client_id"));
        assert!(auth_url.contains("redirect_uri="));
        assert!(auth_url.contains("scope="));
        assert!(auth_url.contains("response_type=code"));
    }

    #[tokio::test]
    #[ignore = "Requires Google OAuth credentials"]
    async fn test_get_user_info() {
        let service = GoogleOAuthService::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:8080/callback".to_string(),
        );

        let result = service.get_user_info("invalid_code").await;

        assert!(result.is_err());
    }
}
