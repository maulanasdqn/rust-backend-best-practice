use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: Uuid,
    /// User email
    pub email: String,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Token type: "access" or "refresh"
    pub token_type: String,
}

impl Claims {
    pub fn new_access_token(user_id: Uuid, email: String, exp: i64) -> Self {
        Self {
            sub: user_id,
            email,
            exp,
            iat: chrono::Utc::now().timestamp(),
            token_type: "access".to_string(),
        }
    }

    pub fn new_refresh_token(user_id: Uuid, email: String, exp: i64) -> Self {
        Self {
            sub: user_id,
            email,
            exp,
            iat: chrono::Utc::now().timestamp(),
            token_type: "refresh".to_string(),
        }
    }

    pub fn is_access_token(&self) -> bool {
        self.token_type == "access"
    }

    pub fn is_refresh_token(&self) -> bool {
        self.token_type == "refresh"
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.exp
    }
}
