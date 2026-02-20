use abbp_types::{Filter, FilterBuilder, FilterValue};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::AccountType;

#[derive(Debug, Clone, Default, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct AccountFilters {
    #[param(required = false)]
    pub user_id: Option<Uuid>,

    #[param(required = false)]
    pub account_type: Option<AccountType>,

    #[param(required = false)]
    pub currency: Option<String>,

    #[param(required = false)]
    pub is_active: Option<bool>,

    #[param(required = false)]
    pub min_balance: Option<i64>,

    #[param(required = false)]
    pub max_balance: Option<i64>,

    #[param(required = false)]
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

    /// Converts filters to paginator-compatible filter format.
    pub fn to_paginator_filters(&self) -> Vec<Filter> {
        let mut filters = Vec::new();

        if let Some(user_id) = self.user_id {
            filters.push(FilterBuilder::eq(
                "user_id",
                FilterValue::String(user_id.to_string()),
            ));
        }

        if let Some(ref account_type) = self.account_type {
            filters.push(FilterBuilder::eq(
                "account_type",
                FilterValue::String(format!("{account_type:?}")),
            ));
        }

        if let Some(ref currency) = self.currency {
            filters.push(FilterBuilder::eq(
                "currency",
                FilterValue::String(currency.clone()),
            ));
        }

        if let Some(is_active) = self.is_active {
            if is_active {
                filters.push(FilterBuilder::is_null("end_date"));
            } else {
                filters.push(FilterBuilder::is_not_null("end_date"));
            }
        }

        if let Some(min_balance) = self.min_balance {
            filters.push(FilterBuilder::gte(
                "balance",
                FilterValue::Int(min_balance),
            ));
        }

        if let Some(max_balance) = self.max_balance {
            filters.push(FilterBuilder::lte(
                "balance",
                FilterValue::Int(max_balance),
            ));
        }

        if let Some(ref name) = self.name {
            filters.push(FilterBuilder::ilike("name", format!("%{name}%")));
        }

        filters
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
