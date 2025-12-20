use fta_types::{PaginationQuery, SortQuery};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use super::filters::AccountFilters;

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct ListAccountsQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    #[serde(flatten)]
    pub sort: SortQuery,

    #[serde(flatten)]
    pub filters: AccountFilters,
}

pub const ALLOWED_SORT_FIELDS: &[&str] = &[
    "name",
    "account_type",
    "balance",
    "currency",
    "created_at",
    "updated_at",
];
