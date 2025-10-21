use async_trait::async_trait;
use uuid::Uuid;

use super::Account;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(&self, account: Account) -> anyhow::Result<Account>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Account>>;

    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::AccountFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Account>>;

    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::AccountFilters,
    ) -> anyhow::Result<i64>;

    async fn update(&self, account: Account) -> anyhow::Result<Account>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
}
