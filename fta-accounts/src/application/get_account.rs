use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Account, AccountRepository};

pub struct GetAccount {
    repository: Arc<dyn AccountRepository>,
}

impl std::fmt::Debug for GetAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetAccount").finish()
    }
}

impl GetAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Account, AppError> {
        self.repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))
    }
}
