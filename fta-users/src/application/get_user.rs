use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{User, UserRepository};

pub struct GetUser {
    repository: Arc<dyn UserRepository>,
}

impl std::fmt::Debug for GetUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetUser").finish()
    }
}

impl GetUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<User, AppError> {
        self.repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }
}
