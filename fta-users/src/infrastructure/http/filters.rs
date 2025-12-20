use chrono::{DateTime, Utc};
use fta_types::{Filter, FilterBuilder, FilterValue};
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

    pub fn to_paginator_filters(&self) -> Vec<Filter> {
        let mut filters = Vec::new();

        if let Some(ref email) = self.email {
            filters.push(FilterBuilder::ilike("email", format!("%{email}%")));
        }

        if let Some(ref first_name) = self.first_name {
            filters.push(FilterBuilder::ilike("first_name", format!("%{first_name}%")));
        }

        if let Some(ref last_name) = self.last_name {
            filters.push(FilterBuilder::ilike("last_name", format!("%{last_name}%")));
        }

        if let Some(verified) = self.verified_only {
            filters.push(FilterBuilder::eq(
                "email_verified",
                FilterValue::Bool(verified),
            ));
        }

        if let Some(ref provider) = self.oauth_provider {
            filters.push(FilterBuilder::eq(
                "oauth_provider",
                FilterValue::String(provider.clone()),
            ));
        }

        if let Some(created_after) = self.created_after {
            filters.push(FilterBuilder::gte(
                "created_at",
                FilterValue::String(created_after.to_rfc3339()),
            ));
        }

        if let Some(created_before) = self.created_before {
            filters.push(FilterBuilder::lte(
                "created_at",
                FilterValue::String(created_before.to_rfc3339()),
            ));
        }

        if let Some(two_factor) = self.two_factor_enabled {
            filters.push(FilterBuilder::eq(
                "two_factor_enabled",
                FilterValue::Bool(two_factor),
            ));
        }

        filters
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

    #[test]
    fn test_user_filters_to_paginator_filters() {
        let filters = UserFilters {
            email: Some("john@example.com".to_string()),
            verified_only: Some(true),
            two_factor_enabled: Some(false),
            ..Default::default()
        };

        let paginator_filters = filters.to_paginator_filters();
        assert_eq!(paginator_filters.len(), 3);
    }

    #[test]
    fn test_empty_filters_to_paginator() {
        let filters = UserFilters::default();
        let paginator_filters = filters.to_paginator_filters();
        assert!(paginator_filters.is_empty());
    }
}
