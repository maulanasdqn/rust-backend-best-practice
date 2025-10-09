use crate::infrastructure::services::GoogleOAuthService;

/// Use case for initiating Google OAuth login
pub struct GoogleOAuthLogin {
    google_oauth_service: GoogleOAuthService,
}

impl GoogleOAuthLogin {
    pub fn new(google_oauth_service: GoogleOAuthService) -> Self {
        Self {
            google_oauth_service,
        }
    }

    /// Generates the Google OAuth authorization URL
    ///
    /// The user should be redirected to this URL to authenticate with Google
    ///
    /// # Returns
    /// The authorization URL as a string
    pub fn execute(&self) -> String {
        self.google_oauth_service.get_authorization_url()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_returns_url() {
        let service = GoogleOAuthService::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:8080/callback".to_string(),
        );
        let use_case = GoogleOAuthLogin::new(service);

        let url = use_case.execute();

        assert!(!url.is_empty());
        assert!(url.contains("https://accounts.google.com"));
    }
}
