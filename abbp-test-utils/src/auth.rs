use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct TestAuth {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
}

impl TestAuth {
    pub fn new(user_id: Uuid, access_token: String, refresh_token: String) -> Self {
        Self {
            user_id,
            access_token,
            refresh_token,
        }
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestRegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl TestRegisterRequest {
    pub fn new(email: String, password: String) -> Self {
        Self {
            email,
            password,
            first_name: None,
            last_name: None,
        }
    }

    pub fn with_names(mut self, first_name: String, last_name: String) -> Self {
        self.first_name = Some(first_name);
        self.last_name = Some(last_name);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestLoginRequest {
    pub email: String,
    pub password: String,
}

impl TestLoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestLoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: Uuid,
}

pub async fn create_test_user_and_login(
    client: &crate::http_client::TestClient,
    email: &str,
    password: &str,
) -> TestAuth {
    let register_req = TestRegisterRequest::new(email.to_string(), password.to_string());
    let _register_response = client.post("/api/v1/auth/register", &register_req).await;

    let login_req = TestLoginRequest::new(email.to_string(), password.to_string());
    let login_response: TestLoginResponse = client
        .post_json("/api/v1/auth/login", &login_req)
        .await
        .expect("Failed to login test user");

    TestAuth::new(
        login_response.user_id,
        login_response.access_token,
        login_response.refresh_token,
    )
}
