use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::AccountType;

#[derive(Debug, Clone, Default, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct AccountFilters {
    pub user_id: Option<Uuid>,

    pub account_type: Option<AccountType>,

    pub currency: Option<String>,

    pub is_active: Option<bool>,

    pub min_balance: Option<i64>,

    pub max_balance: Option<i64>,

    pub name: Option<String>,
}

impl AccountFilters {
    pub const fn is_empty(&self) -> bool {
        self.user_id.is_none()
            && self.account_type.is_none()
            && self.currency.is_none()
            && self.is_active.is_none()
            && self.min_balance.is_none()
            && self.max_balance.is_none()
            && self.name.is_none()
    }

    pub fn validate(&self) -> Result<(), String> {
        if let (Some(min), Some(max)) = (self.min_balance, self.max_balance) {
            if min > max {
                return Err("min_balance must be less than or equal to max_balance".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_filters_is_empty() {
        let filters = AccountFilters::default();
        assert!(filters.is_empty());

        let filters = AccountFilters {
            currency: Some("USD".to_string()),
            ..Default::default()
        };
        assert!(!filters.is_empty());
    }

    #[test]
    fn test_account_filters_validation() {
        let filters = AccountFilters {
            min_balance: Some(1000),
            max_balance: Some(500),
            ..Default::default()
        };
        assert!(filters.validate().is_err());

        let filters = AccountFilters {
            min_balance: Some(500),
            max_balance: Some(1000),
            ..Default::default()
        };
        assert!(filters.validate().is_ok());
    }
}
