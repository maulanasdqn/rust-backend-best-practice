use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{User, UserRepository};

pub struct UpdateUser {
    repository: Arc<dyn UserRepository>,
}

impl std::fmt::Debug for UpdateUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateUser").finish()
    }
}

impl UpdateUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, AppError> {
        let mut user = self
            .repository
            .find_by_id(&id)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        user.update_profile(first_name, last_name);

        self.repository.update(user).await.map_err(AppError::from)
    }
}
