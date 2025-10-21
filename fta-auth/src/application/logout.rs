use anyhow::Context;
use std::sync::Arc;

use crate::domain::RefreshTokenRepository;

pub struct Logout {
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
}

impl std::fmt::Debug for Logout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Logout")
            .field(
                "refresh_token_repository",
                &"Arc<dyn RefreshTokenRepository>",
            )
            .finish()
    }
}

impl Logout {
    pub fn new(refresh_token_repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self {
            refresh_token_repository,
        }
    }

    pub async fn execute(&self, refresh_token: String) -> anyhow::Result<()> {
        self.refresh_token_repository
            .delete_by_token(&refresh_token)
            .await
            .context("Failed to delete refresh token")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
