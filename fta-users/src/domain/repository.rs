use async_trait::async_trait;
use uuid::Uuid;

use super::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> anyhow::Result<User>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;

    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::UserFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<User>>;

    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::UserFilters,
    ) -> anyhow::Result<i64>;

    async fn update(&self, user: User) -> anyhow::Result<User>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
}
