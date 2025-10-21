use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Default, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct UserFilters {
    pub email: Option<String>,

    pub first_name: Option<String>,

    pub last_name: Option<String>,

    pub verified_only: Option<bool>,

    pub oauth_provider: Option<String>,

    pub created_after: Option<DateTime<Utc>>,

    pub created_before: Option<DateTime<Utc>>,

    pub two_factor_enabled: Option<bool>,
}

impl UserFilters {
    pub const fn is_empty(&self) -> bool {
        self.email.is_none()
            && self.first_name.is_none()
            && self.last_name.is_none()
            && self.verified_only.is_none()
            && self.oauth_provider.is_none()
            && self.created_after.is_none()
            && self.created_before.is_none()
            && self.two_factor_enabled.is_none()
    }

    pub fn validate(&self) -> Result<(), String> {
        if let (Some(after), Some(before)) = (self.created_after, self.created_before) {
            if after > before {
                return Err("created_after must be before created_before".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_filters_is_empty() {
        let filters = UserFilters::default();
        assert!(filters.is_empty());

        let filters = UserFilters {
            email: Some("test@example.com".to_string()),
            ..Default::default()
        };
        assert!(!filters.is_empty());
    }

    #[test]
    fn test_user_filters_validation() {
        let filters = UserFilters {
            created_after: Some(Utc::now()),
            created_before: Some(Utc::now() - chrono::Duration::days(1)),
            ..Default::default()
        };
        assert!(filters.validate().is_err());

        let filters = UserFilters {
            created_after: Some(Utc::now() - chrono::Duration::days(1)),
            created_before: Some(Utc::now()),
            ..Default::default()
        };
        assert!(filters.validate().is_ok());
    }
}
