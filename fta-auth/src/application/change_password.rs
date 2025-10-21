use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::PasswordHashService;

pub struct ChangePassword {
    user_repository: Arc<dyn UserRepository>,
    password_hash_service: PasswordHashService,
}

impl std::fmt::Debug for ChangePassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChangePassword")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field("password_hash_service", &self.password_hash_service)
            .finish()
    }
}

impl ChangePassword {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hash_service: PasswordHashService,
    ) -> Self {
        Self {
            user_repository,
            password_hash_service,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        current_password: String,
        new_password: String,
    ) -> anyhow::Result<()> {
        let mut user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let is_valid = self
            .password_hash_service
            .verify_password(&current_password, &user.password_hash)?;

        if !is_valid {
            return Err(anyhow::anyhow!("Current password is incorrect"));
        }

        let new_password_hash = self
            .password_hash_service
            .hash_password(&new_password)
            .context("Failed to hash password")?;

        user.change_password(new_password_hash);

        self.user_repository
            .update(user)
            .await
            .context("Failed to update user password")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
