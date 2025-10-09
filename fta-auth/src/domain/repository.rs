use async_trait::async_trait;
use uuid::Uuid;

use super::{EmailVerification, PasswordResetToken, RefreshToken};

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, refresh_token: RefreshToken) -> anyhow::Result<RefreshToken>;
    async fn find_by_token(&self, token: &str) -> anyhow::Result<Option<RefreshToken>>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Vec<RefreshToken>>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
    async fn delete_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<()>;
    async fn delete_by_token(&self, token: &str) -> anyhow::Result<()>;
    async fn delete_expired(&self) -> anyhow::Result<u64>;
}

#[async_trait]
pub trait EmailVerificationRepository: Send + Sync {
    async fn create(&self, verification: EmailVerification) -> anyhow::Result<EmailVerification>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Option<EmailVerification>>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
    async fn delete_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<()>;
    async fn delete_expired(&self) -> anyhow::Result<u64>;
}

#[async_trait]
pub trait PasswordResetTokenRepository: Send + Sync {
    async fn create(&self, token: PasswordResetToken) -> anyhow::Result<PasswordResetToken>;
    async fn find_by_token(&self, token: &str) -> anyhow::Result<Option<PasswordResetToken>>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Vec<PasswordResetToken>>;
    async fn update(&self, token: PasswordResetToken) -> anyhow::Result<PasswordResetToken>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
    async fn delete_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<()>;
    async fn delete_expired(&self) -> anyhow::Result<u64>;
}
