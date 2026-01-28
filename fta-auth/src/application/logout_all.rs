//! Logout all sessions use case.

use fta_errors::AppError;
use fta_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::RefreshTokenRepository;

/// Logs out a user from all sessions by revoking all their refresh tokens.
pub struct LogoutAll {
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
}

impl_use_case_debug!(LogoutAll);

impl LogoutAll {
    pub fn new(refresh_token_repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self {
            refresh_token_repository,
        }
    }

    /// Revokes all refresh tokens for the specified user.
    ///
    /// # Errors
    /// Returns `InternalError` if token deletion fails.
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn execute(&self, user_id: Uuid) -> Result<(), AppError> {
        tracing::debug!("Logging out user from all sessions");

        self.refresh_token_repository
            .delete_by_user_id(&user_id)
            .await?;

        tracing::info!("User logged out from all sessions");
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
