use fta_errors::AppError;
use paginator_rs::PaginationParams;
use std::sync::Arc;

use crate::domain::{User, UserRepository};

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

    pub async fn execute(&self, params: &PaginationParams) -> Result<(Vec<User>, i64), AppError> {
        let total = self.repository.count_all().await.map_err(AppError::from)?;
        let users = self
            .repository
            .find_all(i64::from(params.limit()), i64::from(params.offset()))
            .await
            .map_err(AppError::from)?;
        Ok((users, total))
    }
}
