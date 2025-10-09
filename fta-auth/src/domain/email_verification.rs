use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub otp_code: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl EmailVerification {
    pub fn new(user_id: Uuid, otp_code: String, validity_minutes: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            otp_code,
            expires_at: now + chrono::Duration::minutes(validity_minutes),
            created_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_valid(&self, provided_otp: &str) -> bool {
        !self.is_expired() && self.otp_code == provided_otp
    }
}
