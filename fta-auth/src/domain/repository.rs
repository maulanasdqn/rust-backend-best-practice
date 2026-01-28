//! Repository traits for authentication-related persistence operations.

use async_trait::async_trait;
use fta_errors::AppError;
use uuid::Uuid;

use super::{EmailVerification, PasswordResetToken, RefreshToken};

/// Repository trait for refresh token persistence operations.
#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    /// Creates a new refresh token.
    async fn create(&self, refresh_token: RefreshToken) -> Result<RefreshToken, AppError>;

    /// Finds a refresh token by its token string.
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError>;

    /// Finds all refresh tokens for a user.
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<RefreshToken>, AppError>;

    /// Deletes a refresh token by its ID.
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;

    /// Deletes all refresh tokens for a user.
    async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), AppError>;

    /// Deletes a refresh token by its token string.
    async fn delete_by_token(&self, token: &str) -> Result<(), AppError>;

    /// Deletes all expired refresh tokens, returning the count deleted.
    async fn delete_expired(&self) -> Result<u64, AppError>;
}

/// Repository trait for email verification persistence operations.
#[async_trait]
pub trait EmailVerificationRepository: Send + Sync {
    /// Creates a new email verification record.
    async fn create(&self, verification: EmailVerification) -> Result<EmailVerification, AppError>;

    /// Finds an email verification by user ID.
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Option<EmailVerification>, AppError>;

    /// Deletes an email verification by its ID.
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;

    /// Deletes all email verifications for a user.
    async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), AppError>;

    /// Deletes all expired email verifications, returning the count deleted.
    async fn delete_expired(&self) -> Result<u64, AppError>;
}

/// Repository trait for password reset token persistence operations.
#[async_trait]
pub trait PasswordResetTokenRepository: Send + Sync {
    /// Creates a new password reset token.
    async fn create(&self, token: PasswordResetToken) -> Result<PasswordResetToken, AppError>;

    /// Finds a password reset token by its token string.
    async fn find_by_token(&self, token: &str) -> Result<Option<PasswordResetToken>, AppError>;

    /// Finds all password reset tokens for a user.
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<PasswordResetToken>, AppError>;

    /// Updates an existing password reset token.
    async fn update(&self, token: PasswordResetToken) -> Result<PasswordResetToken, AppError>;

    /// Deletes a password reset token by its ID.
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;

    /// Deletes all password reset tokens for a user.
    async fn delete_by_user_id(&self, user_id: &Uuid) -> Result<(), AppError>;

    /// Deletes all expired password reset tokens, returning the count deleted.
    async fn delete_expired(&self) -> Result<u64, AppError>;
}
