pub use paginator_rs::PaginationParams as PaginatorParams;
pub use paginator_utils::{
    Cursor, CursorDirection, CursorValue, Filter, FilterOperator, FilterValue,
    PaginatorResponse, PaginatorResponseMeta, SearchParams, SortDirection as PaginatorSortDirection,
};

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct PaginationBuilder {
    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_per_page")]
    pub per_page: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_fields: Option<String>,

    #[serde(default)]
    pub disable_total_count: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PaginationBuilder {
    pub const fn new(page: u32, per_page: u32) -> Self {
        Self {
            page,
            per_page,
            sort_by: None,
            sort_direction: None,
            search: None,
            search_fields: None,
            disable_total_count: false,
            cursor: None,
        }
    }

    #[must_use]
    pub fn build(&self) -> PaginatorParams {
        let mut params = PaginatorParams::new(self.page, self.per_page);

        if let Some(ref sort_by) = self.sort_by {
            params = params.with_sort(sort_by.clone());
        }

        if let Some(ref direction_str) = self.sort_direction {
            let direction = match direction_str.to_uppercase().as_str() {
                "DESC" => PaginatorSortDirection::Desc,
                _ => PaginatorSortDirection::Asc,
            };
            params = params.with_direction(direction);
        }

        if let (Some(ref query), Some(ref fields_str)) = (&self.search, &self.search_fields) {
            let fields: Vec<String> = fields_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            if !query.is_empty() && !fields.is_empty() {
                params = params.with_search(SearchParams::new(query.clone(), fields));
            }
        }

        if self.disable_total_count {
            params.disable_total_count = true;
        }

        if let Some(ref cursor_str) = self.cursor {
            if let Ok(cursor) = Cursor::decode(cursor_str) {
                params.cursor = Some(cursor);
            }
        }

        params
    }

    #[must_use]
    pub fn build_with_filters(self, filters: Vec<Filter>) -> PaginatorParams {
        self.build().with_filters(filters)
    }
}

const fn default_page() -> u32 {
    1
}

const fn default_per_page() -> u32 {
    10
}

#[derive(Debug)]
pub struct FilterBuilder;

impl FilterBuilder {
    pub fn eq(field: impl Into<String>, value: FilterValue) -> Filter {
        Filter::new(field, FilterOperator::Eq, value)
    }

    pub fn ne(field: impl Into<String>, value: FilterValue) -> Filter {
        Filter::new(field, FilterOperator::Ne, value)
    }

    pub fn gt(field: impl Into<String>, value: FilterValue) -> Filter {
        Filter::new(field, FilterOperator::Gt, value)
    }

    pub fn lt(field: impl Into<String>, value: FilterValue) -> Filter {
        Filter::new(field, FilterOperator::Lt, value)
    }

    pub fn gte(field: impl Into<String>, value: FilterValue) -> Filter {
        Filter::new(field, FilterOperator::Gte, value)
    }

    pub fn lte(field: impl Into<String>, value: FilterValue) -> Filter {
        Filter::new(field, FilterOperator::Lte, value)
    }

    pub fn like(field: impl Into<String>, value: impl Into<String>) -> Filter {
        Filter::new(field, FilterOperator::Like, FilterValue::String(value.into()))
    }

    pub fn ilike(field: impl Into<String>, value: impl Into<String>) -> Filter {
        Filter::new(
            field,
            FilterOperator::ILike,
            FilterValue::String(value.into()),
        )
    }

    pub fn in_array(field: impl Into<String>, values: Vec<FilterValue>) -> Filter {
        Filter::new(field, FilterOperator::In, FilterValue::Array(values))
    }

    pub fn not_in(field: impl Into<String>, values: Vec<FilterValue>) -> Filter {
        Filter::new(field, FilterOperator::NotIn, FilterValue::Array(values))
    }

    pub fn is_null(field: impl Into<String>) -> Filter {
        Filter::new(field, FilterOperator::IsNull, FilterValue::Null)
    }

    pub fn is_not_null(field: impl Into<String>) -> Filter {
        Filter::new(field, FilterOperator::IsNotNull, FilterValue::Null)
    }

    pub fn between(field: impl Into<String>, start: FilterValue, end: FilterValue) -> Filter {
        Filter::new(
            field,
            FilterOperator::Between,
            FilterValue::Array(vec![start, end]),
        )
    }

    pub fn contains(field: impl Into<String>, value: FilterValue) -> Filter {
        Filter::new(field, FilterOperator::Contains, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_builder_basic() {
        let builder = PaginationBuilder::new(2, 25);
        let params = builder.build();
        assert_eq!(params.page, 2);
        assert_eq!(params.per_page, 25);
    }

    #[test]
    fn test_pagination_builder_with_sort() {
        let mut builder = PaginationBuilder::new(1, 10);
        builder.sort_by = Some("email".to_string());
        builder.sort_direction = Some("desc".to_string());
        let params = builder.build();
        assert_eq!(params.sort_by, Some("email".to_string()));
    }

    #[test]
    fn test_pagination_builder_with_search() {
        let mut builder = PaginationBuilder::new(1, 10);
        builder.search = Some("john".to_string());
        builder.search_fields = Some("name,email".to_string());
        let params = builder.build();
        assert!(params.search.is_some());
    }

    #[test]
    fn test_filter_builder_eq() {
        let filter = FilterBuilder::eq("status", FilterValue::String("active".to_string()));
        assert_eq!(filter.field, "status");
        assert_eq!(filter.operator, FilterOperator::Eq);
    }

    #[test]
    fn test_filter_builder_between() {
        let filter = FilterBuilder::between("age", FilterValue::Int(18), FilterValue::Int(65));
        assert_eq!(filter.operator, FilterOperator::Between);
        if let FilterValue::Array(values) = &filter.value {
            assert_eq!(values.len(), 2);
        } else {
            panic!("Expected Array");
        }
    }

    #[test]
    fn test_filter_builder_in_array() {
        let values = vec![
            FilterValue::String("admin".to_string()),
            FilterValue::String("user".to_string()),
        ];
        let filter = FilterBuilder::in_array("role", values);
        assert_eq!(filter.operator, FilterOperator::In);
    }
}
