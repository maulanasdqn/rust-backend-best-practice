use async_trait::async_trait;
use uuid::Uuid;

use super::Transaction;

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, transaction: Transaction) -> anyhow::Result<Transaction>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Transaction>>;

    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::TransactionFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Transaction>>;

    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::TransactionFilters,
    ) -> anyhow::Result<i64>;

    async fn update(&self, transaction: Transaction) -> anyhow::Result<Transaction>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
}
