use anyhow::Context;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::RefreshTokenRepository;

pub struct LogoutAll {
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
}

impl std::fmt::Debug for LogoutAll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogoutAll")
            .field(
                "refresh_token_repository",
                &"Arc<dyn RefreshTokenRepository>",
            )
            .finish()
    }
}

impl LogoutAll {
    pub fn new(refresh_token_repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self {
            refresh_token_repository,
        }
    }

    pub async fn execute(&self, user_id: Uuid) -> anyhow::Result<()> {
        self.refresh_token_repository
            .delete_by_user_id(&user_id)
            .await
            .context("Failed to delete all refresh tokens")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
