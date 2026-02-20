//! # FTA Types
//!
//! Shared types, utilities, and macros for the Financial Tracker API.
//!
//! This crate provides:
//! - Common data types (`Money`, `DateRange`)
//! - Pagination utilities
//! - API response types (`SingleResponse`, `ListResponse`, `ErrorResponse`)
//! - Query parameter handling
//! - Utility macros for use cases

pub mod date_range;
pub mod money;
pub mod openapi_examples;
pub mod pagination;
pub mod query_params;
pub mod responses;

/// Macro to implement `Debug` for use case structs.
///
/// Use cases typically contain `Arc<dyn Trait>` fields which cannot derive `Debug`.
/// This macro provides a consistent, minimal Debug implementation.
///
/// # Example
///
/// ```ignore
/// use abbp_types::impl_use_case_debug;
///
/// pub struct CreateAccount {
///     repository: Arc<dyn AccountRepository>,
/// }
///
/// impl_use_case_debug!(CreateAccount);
/// ```
#[macro_export]
macro_rules! impl_use_case_debug {
    ($name:ident) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
}

pub use date_range::DateRange;
pub use money::Money;
pub use pagination::{
    Cursor, CursorDirection, CursorValue, Filter, FilterBuilder, FilterOperator, FilterValue,
    PaginationBuilder, PaginatorParams, PaginatorResponse, PaginatorResponseMeta,
    PaginatorSortDirection, SearchParams,
};
pub use query_params::{ListQuery, PaginationQuery, SortOrder, SortQuery, MAX_PER_PAGE};
pub use responses::{ErrorResponse, ListResponse, MessageOnlyResponse, PaginationMeta, SingleResponse};
