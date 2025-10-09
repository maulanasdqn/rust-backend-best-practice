use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::Transaction;

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, transaction: Transaction) -> anyhow::Result<Transaction>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Transaction>>;
    async fn find_by_account_id(
        &self,
        account_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Transaction>>;
    async fn count_by_account_id(&self, account_id: &Uuid) -> anyhow::Result<i64>;
    async fn find_by_date_range(
        &self,
        account_id: &Uuid,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> anyhow::Result<Vec<Transaction>>;
    async fn update(&self, transaction: Transaction) -> anyhow::Result<Transaction>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
}
