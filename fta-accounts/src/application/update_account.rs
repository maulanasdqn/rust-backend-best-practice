use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Account, AccountRepository};

pub struct UpdateAccount {
    repository: Arc<dyn AccountRepository>,
}

impl std::fmt::Debug for UpdateAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateAccount").finish()
    }
}

impl UpdateAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid, name: Option<String>) -> Result<Account, AppError> {
        let mut account = self
            .repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

        if let Some(new_name) = name {
            account.rename(new_name);
        }

        self.repository
            .update(account)
            .await
            .map_err(AppError::from)
    }
}
