use anyhow::Result;
use async_trait::async_trait;
use fta_database::{entities::budgets, sea_orm, DbPool};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::domain::{Budget, BudgetPeriod, BudgetRepository};
use crate::infrastructure::http::filters::BudgetFilters;

fn model_to_budget(model: budgets::Model) -> Budget {
    Budget {
        id: model.id,
        user_id: model.user_id,
        category: model.category,
        amount: model.amount,
        period: serde_json::from_str(&model.period).unwrap_or(BudgetPeriod::Monthly),
        start_date: model.start_date,
        end_date: model.end_date,
        is_active: model.is_active,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

#[derive(Clone, Debug)]
pub struct PostgresBudgetRepository {
    pool: DbPool,
}

impl PostgresBudgetRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BudgetRepository for PostgresBudgetRepository {
    async fn create(&self, budget: Budget) -> Result<Budget> {
        let period_str = serde_json::to_string(&budget.period)?;

        let active_model = budgets::ActiveModel {
            id: Set(budget.id),
            user_id: Set(budget.user_id),
            category: Set(budget.category.clone()),
            amount: Set(budget.amount),
            period: Set(period_str),
            start_date: Set(budget.start_date),
            end_date: Set(budget.end_date),
            is_active: Set(budget.is_active),
            created_at: Set(budget.created_at),
            updated_at: Set(budget.updated_at),
        };

        let result = active_model.insert(&self.pool).await?;
        Ok(model_to_budget(result))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Budget>> {
        let result = budgets::Entity::find_by_id(*id).one(&self.pool).await?;
        Ok(result.map(model_to_budget))
    }

    async fn find_all(
        &self,
        filters: &BudgetFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Budget>> {
        let mut query = budgets::Entity::find();

        // Apply filters
        if let Some(user_id) = filters.user_id {
            query = query.filter(budgets::Column::UserId.eq(user_id));
        }
        if let Some(ref category) = filters.category {
            query = query.filter(budgets::Column::Category.eq(category));
        }
        if let Some(ref period) = filters.period {
            let period_str = serde_json::to_string(period).unwrap_or_default();
            query = query.filter(budgets::Column::Period.eq(period_str));
        }
        if let Some(is_active) = filters.is_active {
            query = query.filter(budgets::Column::IsActive.eq(is_active));
        }
        if let Some(min_amount) = filters.min_amount {
            query = query.filter(budgets::Column::Amount.gte(min_amount));
        }
        if let Some(max_amount) = filters.max_amount {
            query = query.filter(budgets::Column::Amount.lte(max_amount));
        }
        if let Some(start_after) = filters.start_after {
            query = query.filter(budgets::Column::StartDate.gte(start_after));
        }
        if let Some(start_before) = filters.start_before {
            query = query.filter(budgets::Column::StartDate.lte(start_before));
        }

        // Apply sorting
        let order = if sort_order.to_lowercase() == "asc" {
            sea_orm::Order::Asc
        } else {
            sea_orm::Order::Desc
        };

        query = match sort_by {
            Some("category") => query.order_by(budgets::Column::Category, order),
            Some("amount") => query.order_by(budgets::Column::Amount, order),
            Some("start_date") => query.order_by(budgets::Column::StartDate, order),
            Some("created_at") | None => query.order_by(budgets::Column::CreatedAt, order),
            Some(_) => query.order_by(budgets::Column::CreatedAt, order),
        };

        let results = query
            .paginate(&self.pool, limit as u64)
            .fetch_page((offset / limit.max(1)) as u64)
            .await?;

        Ok(results.into_iter().map(model_to_budget).collect())
    }

    async fn count_all(&self, filters: &BudgetFilters) -> Result<i64> {
        let mut query = budgets::Entity::find();

        // Apply filters
        if let Some(user_id) = filters.user_id {
            query = query.filter(budgets::Column::UserId.eq(user_id));
        }
        if let Some(ref category) = filters.category {
            query = query.filter(budgets::Column::Category.eq(category));
        }
        if let Some(ref period) = filters.period {
            let period_str = serde_json::to_string(period).unwrap_or_default();
            query = query.filter(budgets::Column::Period.eq(period_str));
        }
        if let Some(is_active) = filters.is_active {
            query = query.filter(budgets::Column::IsActive.eq(is_active));
        }
        if let Some(min_amount) = filters.min_amount {
            query = query.filter(budgets::Column::Amount.gte(min_amount));
        }
        if let Some(max_amount) = filters.max_amount {
            query = query.filter(budgets::Column::Amount.lte(max_amount));
        }
        if let Some(start_after) = filters.start_after {
            query = query.filter(budgets::Column::StartDate.gte(start_after));
        }
        if let Some(start_before) = filters.start_before {
            query = query.filter(budgets::Column::StartDate.lte(start_before));
        }

        let count = query.count(&self.pool).await?;
        Ok(count as i64)
    }

    async fn update(&self, budget: Budget) -> Result<Budget> {
        let period_str = serde_json::to_string(&budget.period)?;

        let active_model = budgets::ActiveModel {
            id: Set(budget.id),
            user_id: Set(budget.user_id),
            category: Set(budget.category.clone()),
            amount: Set(budget.amount),
            period: Set(period_str),
            start_date: Set(budget.start_date),
            end_date: Set(budget.end_date),
            is_active: Set(budget.is_active),
            created_at: Set(budget.created_at),
            updated_at: Set(budget.updated_at),
        };

        let result = active_model.update(&self.pool).await?;
        Ok(model_to_budget(result))
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        budgets::Entity::delete_by_id(*id).exec(&self.pool).await?;
        Ok(())
    }
}
