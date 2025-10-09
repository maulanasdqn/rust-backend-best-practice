use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[schema(write_only)]
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_verified: bool,
    pub two_factor_enabled: bool,
    #[schema(write_only)]
    pub two_factor_secret: Option<String>,
    pub oauth_provider: Option<String>,
    pub oauth_provider_id: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        email: String,
        password_hash: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            first_name,
            last_name,
            email_verified: false,
            two_factor_enabled: false,
            two_factor_secret: None,
            oauth_provider: None,
            oauth_provider_id: None,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn full_name(&self) -> Option<String> {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => Some(format!("{first} {last}")),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            (None, None) => None,
        }
    }

    pub fn update_profile(&mut self, first_name: Option<String>, last_name: Option<String>) {
        self.first_name = first_name;
        self.last_name = last_name;
        self.updated_at = Utc::now();
    }

    pub fn change_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
        self.updated_at = Utc::now();
    }
}
