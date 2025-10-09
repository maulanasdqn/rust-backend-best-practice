use async_trait::async_trait;
use uuid::Uuid;

use super::Budget;

#[async_trait]
pub trait BudgetRepository: Send + Sync {
    async fn create(&self, budget: Budget) -> anyhow::Result<Budget>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Budget>>;
    async fn find_by_user_id(
        &self,
        user_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Budget>>;
    async fn count_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<i64>;
    async fn find_active_by_user_id(&self, user_id: &Uuid) -> anyhow::Result<Vec<Budget>>;
    async fn update(&self, budget: Budget) -> anyhow::Result<Budget>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
}
