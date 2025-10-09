use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::AccountRepository;

pub struct DeleteAccount {
    repository: Arc<dyn AccountRepository>,
}

impl std::fmt::Debug for DeleteAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteAccount").finish()
    }
}

impl DeleteAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        self.repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

        self.repository.delete(&id).await.map_err(AppError::from)
    }
}
