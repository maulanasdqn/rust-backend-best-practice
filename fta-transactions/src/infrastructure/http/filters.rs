use chrono::{DateTime, Utc};
use fta_types::{DateRange, Filter, FilterBuilder, FilterValue};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::TransactionType;

#[derive(Debug, Clone, Default, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct TransactionFilters {
    #[param(required = false)]
    pub account_id: Option<Uuid>,

    #[param(required = false)]
    pub transaction_type: Option<TransactionType>,

    #[param(required = false)]
    pub category: Option<String>,

    #[param(required = false)]
    pub min_amount: Option<i64>,

    #[param(required = false)]
    pub max_amount: Option<i64>,

    #[param(required = false)]
    pub start_date: Option<DateTime<Utc>>,

    #[param(required = false)]
    pub end_date: Option<DateTime<Utc>>,

    #[param(required = false)]
    pub search: Option<String>,
}

impl TransactionFilters {
    pub const fn is_empty(&self) -> bool {
        self.account_id.is_none()
            && self.transaction_type.is_none()
            && self.category.is_none()
            && self.min_amount.is_none()
            && self.max_amount.is_none()
            && self.start_date.is_none()
            && self.end_date.is_none()
            && self.search.is_none()
    }

    pub fn validate(&self) -> Result<(), String> {
        if let (Some(min), Some(max)) = (self.min_amount, self.max_amount) {
            if min > max {
                return Err("min_amount must be less than or equal to max_amount".to_string());
            }
        }

        if let (Some(start), Some(end)) = (self.start_date, self.end_date) {
            if start > end {
                return Err("start_date must be before or equal to end_date".to_string());
            }
        }

        Ok(())
    }

    pub fn date_range(&self) -> Option<DateRange> {
        match (self.start_date, self.end_date) {
            (Some(start), Some(end)) => DateRange::new(start, end).ok(),
            _ => None,
        }
    }

    /// Converts filters to paginator-compatible filter format.
    pub fn to_paginator_filters(&self) -> Vec<Filter> {
        let mut filters = Vec::new();

        if let Some(account_id) = self.account_id {
            filters.push(FilterBuilder::eq(
                "account_id",
                FilterValue::String(account_id.to_string()),
            ));
        }

        if let Some(ref transaction_type) = self.transaction_type {
            filters.push(FilterBuilder::eq(
                "transaction_type",
                FilterValue::String(format!("{transaction_type:?}")),
            ));
        }

        if let Some(ref category) = self.category {
            filters.push(FilterBuilder::ilike("category", format!("%{category}%")));
        }

        if let Some(min_amount) = self.min_amount {
            filters.push(FilterBuilder::gte(
                "amount",
                FilterValue::Int(min_amount),
            ));
        }

        if let Some(max_amount) = self.max_amount {
            filters.push(FilterBuilder::lte(
                "amount",
                FilterValue::Int(max_amount),
            ));
        }

        if let Some(start_date) = self.start_date {
            filters.push(FilterBuilder::gte(
                "transaction_date",
                FilterValue::String(start_date.to_rfc3339()),
            ));
        }

        if let Some(end_date) = self.end_date {
            filters.push(FilterBuilder::lte(
                "transaction_date",
                FilterValue::String(end_date.to_rfc3339()),
            ));
        }

        if let Some(ref search) = self.search {
            filters.push(FilterBuilder::ilike("description", format!("%{search}%")));
        }

        filters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_filters_is_empty() {
        let filters = TransactionFilters::default();
        assert!(filters.is_empty());

        let filters = TransactionFilters {
            category: Some("food".to_string()),
            ..Default::default()
        };
        assert!(!filters.is_empty());
    }

    #[test]
    fn test_transaction_filters_validation() {
        let filters = TransactionFilters {
            min_amount: Some(1000),
            max_amount: Some(500),
            ..Default::default()
        };
        assert!(filters.validate().is_err());

        let filters = TransactionFilters {
            start_date: Some(Utc::now()),
            end_date: Some(Utc::now() - chrono::Duration::days(1)),
            ..Default::default()
        };
        assert!(filters.validate().is_err());

        let filters = TransactionFilters {
            min_amount: Some(500),
            max_amount: Some(1000),
            start_date: Some(Utc::now() - chrono::Duration::days(7)),
            end_date: Some(Utc::now()),
            ..Default::default()
        };
        assert!(filters.validate().is_ok());
    }

    #[test]
    fn test_transaction_filters_date_range() {
        let now = Utc::now();
        let yesterday = now - chrono::Duration::days(1);

        let filters = TransactionFilters {
            start_date: Some(yesterday),
            end_date: Some(now),
            ..Default::default()
        };

        let date_range = filters.date_range();
        assert!(date_range.is_some());
        let range = date_range.unwrap();
        assert_eq!(range.start, yesterday);
        assert_eq!(range.end, now);
    }
}
