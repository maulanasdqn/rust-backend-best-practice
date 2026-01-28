//! User logout use case.

use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;

use crate::domain::RefreshTokenRepository;

/// Logs out a user by revoking their refresh token.
pub struct Logout {
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
}

impl_use_case_debug!(Logout);

impl Logout {
    pub fn new(refresh_token_repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self {
            refresh_token_repository,
        }
    }

    /// Revokes the specified refresh token.
    ///
    /// # Errors
    /// Returns `InternalError` if token deletion fails.
    #[instrument(skip(self, refresh_token))]
    pub async fn execute(&self, refresh_token: String) -> Result<(), AppError> {
        tracing::debug!("Logging out user");

        self.refresh_token_repository
            .delete_by_token(&refresh_token)
            .await?;

        tracing::info!("User logged out successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
