use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::BudgetPeriod;

#[derive(Debug, Clone, Default, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct BudgetFilters {
    #[param(required = false)]
    pub user_id: Option<Uuid>,

    #[param(required = false)]
    pub category: Option<String>,

    #[param(required = false)]
    pub period: Option<BudgetPeriod>,

    #[param(required = false)]
    pub is_active: Option<bool>,

    #[param(required = false)]
    pub min_amount: Option<i64>,

    #[param(required = false)]
    pub max_amount: Option<i64>,

    #[param(required = false)]
    pub start_after: Option<DateTime<Utc>>,

    #[param(required = false)]
    pub start_before: Option<DateTime<Utc>>,
}

impl BudgetFilters {
    pub const fn is_empty(&self) -> bool {
        self.user_id.is_none()
            && self.category.is_none()
            && self.period.is_none()
            && self.is_active.is_none()
            && self.min_amount.is_none()
            && self.max_amount.is_none()
            && self.start_after.is_none()
            && self.start_before.is_none()
    }

    pub fn validate(&self) -> Result<(), String> {
        if let (Some(min), Some(max)) = (self.min_amount, self.max_amount) {
            if min > max {
                return Err("min_amount must be less than or equal to max_amount".to_string());
            }
        }

        if let (Some(after), Some(before)) = (self.start_after, self.start_before) {
            if after > before {
                return Err("start_after must be before or equal to start_before".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_filters_is_empty() {
        let filters = BudgetFilters::default();
        assert!(filters.is_empty());

        let filters = BudgetFilters {
            category: Some("food".to_string()),
            ..Default::default()
        };
        assert!(!filters.is_empty());
    }

    #[test]
    fn test_budget_filters_validation() {
        let filters = BudgetFilters {
            min_amount: Some(1000),
            max_amount: Some(500),
            ..Default::default()
        };
        assert!(filters.validate().is_err());

        let filters = BudgetFilters {
            start_after: Some(Utc::now()),
            start_before: Some(Utc::now() - chrono::Duration::days(1)),
            ..Default::default()
        };
        assert!(filters.validate().is_err());

        let filters = BudgetFilters {
            min_amount: Some(500),
            max_amount: Some(1000),
            start_after: Some(Utc::now() - chrono::Duration::days(7)),
            start_before: Some(Utc::now()),
            ..Default::default()
        };
        assert!(filters.validate().is_ok());
    }
}
