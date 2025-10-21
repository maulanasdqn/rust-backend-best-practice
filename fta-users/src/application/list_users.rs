use fta_errors::AppError;
use fta_types::PaginationQuery;
use std::sync::Arc;

use crate::domain::{User, UserRepository};
use crate::infrastructure::http::filters::UserFilters;

pub struct ListUsers {
    repository: Arc<dyn UserRepository>,
}

impl std::fmt::Debug for ListUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListUsers").finish()
    }
}

impl ListUsers {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        filters: &UserFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        pagination: &PaginationQuery,
    ) -> Result<(Vec<User>, i64), AppError> {
        let total = self
            .repository
            .count_all(filters)
            .await
            .map_err(AppError::from)?;
        let users = self
            .repository
            .find_all(
                filters,
                sort_by,
                sort_order,
                i64::from(pagination.limit()),
                i64::from(pagination.offset()),
            )
            .await
            .map_err(AppError::from)?;
        Ok((users, total))
    }
}
