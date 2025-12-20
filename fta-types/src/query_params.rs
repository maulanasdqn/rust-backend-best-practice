use paginator_rs::PaginationParams as PaginatorParams;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,

    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Asc
    }
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "ASC"),
            Self::Desc => write!(f, "DESC"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            per_page: default_per_page(),
        }
    }
}

impl PaginationQuery {
    pub const fn new(page: u32, per_page: u32) -> Self {
        Self { page, per_page }
    }

    #[must_use]
    pub fn validate(self) -> Self {
        Self {
            page: self.page.max(1),
            per_page: self.per_page.clamp(1, MAX_PER_PAGE),
        }
    }

    pub const fn offset(&self) -> u32 {
        (self.page - 1) * self.per_page
    }

    pub const fn limit(&self) -> u32 {
        self.per_page
    }

    #[must_use]
    pub fn to_paginator_params(&self) -> PaginatorParams {
        PaginatorParams::new(self.page, self.per_page)
    }

    #[must_use]
    pub const fn from_paginator_params(params: &PaginatorParams) -> Self {
        Self {
            page: params.page,
            per_page: params.per_page,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct SortQuery {
    pub sort_by: Option<String>,

    #[serde(default)]
    pub order: SortOrder,
}

impl Default for SortQuery {
    fn default() -> Self {
        Self {
            sort_by: None,
            order: SortOrder::Asc,
        }
    }
}

impl SortQuery {
    pub const fn new(sort_by: Option<String>, order: SortOrder) -> Self {
        Self { sort_by, order }
    }

    pub fn validate(&self, allowed_fields: &[&str]) -> Result<(), String> {
        if let Some(ref field) = self.sort_by {
            if !allowed_fields.contains(&field.as_str()) {
                return Err(format!(
                    "Invalid sort field '{}'. Allowed fields: {}",
                    field,
                    allowed_fields.join(", ")
                ));
            }
        }
        Ok(())
    }

    pub fn to_sql(&self) -> Option<(String, String)> {
        self.sort_by
            .as_ref()
            .map(|field| (field.clone(), self.order.to_string()))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ListQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    #[serde(flatten)]
    pub sort: SortQuery,
}

impl ListQuery {
    pub const fn new(pagination: PaginationQuery, sort: SortQuery) -> Self {
        Self { pagination, sort }
    }

    pub fn validate(mut self, allowed_sort_fields: &[&str]) -> Result<Self, String> {
        self.pagination = self.pagination.validate();
        self.sort.validate(allowed_sort_fields)?;
        Ok(self)
    }
}

const fn default_page() -> u32 {
    1
}

const fn default_per_page() -> u32 {
    10
}

pub const MAX_PER_PAGE: u32 = 100;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_query_defaults() {
        let query = PaginationQuery::default();
        assert_eq!(query.page, 1);
        assert_eq!(query.per_page, 10);
    }

    #[test]
    fn test_pagination_query_offset() {
        let query = PaginationQuery::new(1, 10);
        assert_eq!(query.offset(), 0);

        let query = PaginationQuery::new(2, 10);
        assert_eq!(query.offset(), 10);

        let query = PaginationQuery::new(3, 20);
        assert_eq!(query.offset(), 40);
    }

    #[test]
    fn test_pagination_query_validation() {
        let query = PaginationQuery::new(0, 200).validate();
        assert_eq!(query.page, 1);
        assert_eq!(query.per_page, 100);
    }

    #[test]
    fn test_paginator_params_conversion() {
        let query = PaginationQuery::new(2, 25);
        let params = query.to_paginator_params();
        assert_eq!(params.page, 2);
        assert_eq!(params.per_page, 25);

        let converted_back = PaginationQuery::from_paginator_params(&params);
        assert_eq!(converted_back.page, 2);
        assert_eq!(converted_back.per_page, 25);
    }

    #[test]
    fn test_sort_order_display() {
        assert_eq!(SortOrder::Asc.to_string(), "ASC");
        assert_eq!(SortOrder::Desc.to_string(), "DESC");
    }

    #[test]
    fn test_sort_query_validation() {
        let query = SortQuery::new(Some("name".to_string()), SortOrder::Asc);
        assert!(query.validate(&["name", "email", "created_at"]).is_ok());

        let query = SortQuery::new(Some("invalid".to_string()), SortOrder::Asc);
        assert!(query.validate(&["name", "email"]).is_err());

        let query = SortQuery::new(None, SortOrder::Asc);
        assert!(query.validate(&["name"]).is_ok());
    }

    #[test]
    fn test_sort_query_to_sql() {
        let query = SortQuery::new(Some("name".to_string()), SortOrder::Asc);
        let sql = query.to_sql();
        assert_eq!(sql, Some(("name".to_string(), "ASC".to_string())));

        let query = SortQuery::new(None, SortOrder::Desc);
        assert_eq!(query.to_sql(), None);
    }

    #[test]
    fn test_list_query_validation() {
        let query = ListQuery::new(
            PaginationQuery::new(0, 200),
            SortQuery::new(Some("name".to_string()), SortOrder::Asc),
        );

        let validated = query.validate(&["name", "email"]).unwrap();
        assert_eq!(validated.pagination.page, 1);
        assert_eq!(validated.pagination.per_page, 100);
    }

    #[test]
    fn test_list_query_validation_invalid_sort() {
        let query = ListQuery::new(
            PaginationQuery::new(1, 10),
            SortQuery::new(Some("invalid".to_string()), SortOrder::Asc),
        );

        assert!(query.validate(&["name", "email"]).is_err());
    }
}
