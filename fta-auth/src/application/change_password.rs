use anyhow::Context;
use fta_users::domain::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::PasswordHashService;

/// Use case for changing a user's password (when authenticated)
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

    /// Changes a user's password after verifying the current password
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    /// * `current_password` - The current password (for verification)
    /// * `new_password` - The new password (plain text)
    pub async fn execute(
        &self,
        user_id: Uuid,
        current_password: String,
        new_password: String,
    ) -> anyhow::Result<()> {
        // Get the user
        let mut user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Verify current password
        let is_valid = self
            .password_hash_service
            .verify_password(&current_password, &user.password_hash)?;

        if !is_valid {
            return Err(anyhow::anyhow!("Current password is incorrect"));
        }

        // Hash the new password
        let new_password_hash = self
            .password_hash_service
            .hash_password(&new_password)
            .context("Failed to hash password")?;

        // Update the user's password
        user.change_password(new_password_hash);

        self.user_repository
            .update(user)
            .await
            .context("Failed to update user password")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here
}
