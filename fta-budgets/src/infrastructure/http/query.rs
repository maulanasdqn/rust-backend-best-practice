use fta_types::{PaginationQuery, SortQuery};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use super::filters::BudgetFilters;

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct ListBudgetsQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    #[serde(flatten)]
    pub sort: SortQuery,

    #[serde(flatten)]
    pub filters: BudgetFilters,
}

pub const ALLOWED_SORT_FIELDS: &[&str] = &[
    "category",
    "amount",
    "period",
    "start_date",
    "created_at",
    "updated_at",
];
