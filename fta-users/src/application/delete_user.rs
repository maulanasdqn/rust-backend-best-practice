use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::UserRepository;

pub struct DeleteUser {
    repository: Arc<dyn UserRepository>,
}

impl std::fmt::Debug for DeleteUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteUser").finish()
    }
}

impl DeleteUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        // Verify user exists before deleting
        self.repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        self.repository.delete(&id).await.map_err(AppError::from)
    }
}
