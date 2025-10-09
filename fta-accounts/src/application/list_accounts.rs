use fta_errors::AppError;
use paginator_rs::PaginationParams;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Account, AccountRepository};

pub struct ListAccounts {
    repository: Arc<dyn AccountRepository>,
}

impl std::fmt::Debug for ListAccounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListAccounts").finish()
    }
}

impl ListAccounts {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        params: &PaginationParams,
    ) -> Result<(Vec<Account>, i64), AppError> {
        let total = self
            .repository
            .count_by_user_id(&user_id)
            .await
            .map_err(AppError::from)?;
        let accounts = self
            .repository
            .find_by_user_id(
                &user_id,
                i64::from(params.limit()),
                i64::from(params.offset()),
            )
            .await
            .map_err(AppError::from)?;
        Ok((accounts, total))
    }
}
