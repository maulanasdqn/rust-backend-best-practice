use async_trait::async_trait;
use uuid::Uuid;

use super::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> anyhow::Result<User>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
    async fn find_all(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<User>>;
    async fn count_all(&self) -> anyhow::Result<i64>;
    async fn update(&self, user: User) -> anyhow::Result<User>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
}
