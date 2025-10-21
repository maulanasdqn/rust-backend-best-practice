use crate::infrastructure::services::GoogleOAuthService;

#[derive(Debug)]
pub struct GoogleOAuthLogin {
    google_oauth_service: GoogleOAuthService,
}

impl GoogleOAuthLogin {
    pub const fn new(google_oauth_service: GoogleOAuthService) -> Self {
        Self {
            google_oauth_service,
        }
    }

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
