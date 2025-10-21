use async_trait::async_trait;
use uuid::Uuid;

use super::Budget;

#[async_trait]
pub trait BudgetRepository: Send + Sync {
    async fn create(&self, budget: Budget) -> anyhow::Result<Budget>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Budget>>;

    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::BudgetFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Budget>>;

    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::BudgetFilters,
    ) -> anyhow::Result<i64>;

    async fn update(&self, budget: Budget) -> anyhow::Result<Budget>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<()>;
}
