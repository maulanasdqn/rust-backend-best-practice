use async_trait::async_trait;
use uuid::Uuid;

use super::Account;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(&self, account: Account) -> anyhow::Result<Account>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Account>>;
    async fn find_by_user_id(
        &self,
        user_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Account>>;
    async fn count_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<i64>;
    async fn update(&self, account: Account) -> anyhow::Result<Account>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
}
