use anyhow::Result;
use async_trait::async_trait;
use fta_database::DbPool;
use sqlx::Row;
use uuid::Uuid;

use crate::domain::{Budget, BudgetPeriod, BudgetRepository};

#[derive(Clone, Debug)]
pub struct PostgresBudgetRepository {
    pool: DbPool,
}

impl PostgresBudgetRepository {
    pub const fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BudgetRepository for PostgresBudgetRepository {
    async fn create(&self, budget: Budget) -> Result<Budget> {
        let period_str = serde_json::to_string(&budget.period)?;
        let result = sqlx::query("INSERT INTO budgets (id, user_id, category, amount, period, start_date, end_date, is_active, created_at, updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING *")
      .bind(budget.id).bind(budget.user_id).bind(&budget.category).bind(budget.amount).bind(period_str).bind(budget.start_date).bind(budget.end_date).bind(budget.is_active).bind(budget.created_at).bind(budget.updated_at).fetch_one(&self.pool).await?;
        Ok(Budget {
            id: result.get("id"),
            user_id: result.get("user_id"),
            category: result.get("category"),
            amount: result.get("amount"),
            period: serde_json::from_str(result.get("period"))?,
            start_date: result.get("start_date"),
            end_date: result.get("end_date"),
            is_active: result.get("is_active"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Budget>> {
        let result = sqlx::query("SELECT * FROM budgets WHERE id=$1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(result.map(|r| Budget {
            id: r.get("id"),
            user_id: r.get("user_id"),
            category: r.get("category"),
            amount: r.get("amount"),
            period: serde_json::from_str(r.get("period")).unwrap_or(BudgetPeriod::Monthly),
            start_date: r.get("start_date"),
            end_date: r.get("end_date"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::BudgetFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Budget>> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, user_id, category, amount, period, start_date, end_date, is_active, created_at, updated_at \
             FROM budgets WHERE 1=1",
        );

        if let Some(user_id) = filters.user_id {
            query_builder.push(" AND user_id = ");
            query_builder.push_bind(user_id);
        }

        if let Some(ref category) = filters.category {
            query_builder.push(" AND category = ");
            query_builder.push_bind(category);
        }

        if let Some(ref period) = filters.period {
            let period_str = serde_json::to_string(period).unwrap_or_default();
            query_builder.push(" AND period = ");
            query_builder.push_bind(period_str);
        }

        if let Some(is_active) = filters.is_active {
            query_builder.push(" AND is_active = ");
            query_builder.push_bind(is_active);
        }

        if let Some(min_amount) = filters.min_amount {
            query_builder.push(" AND amount >= ");
            query_builder.push_bind(min_amount);
        }

        if let Some(max_amount) = filters.max_amount {
            query_builder.push(" AND amount <= ");
            query_builder.push_bind(max_amount);
        }

        if let Some(start_after) = filters.start_after {
            query_builder.push(" AND start_date >= ");
            query_builder.push_bind(start_after);
        }

        if let Some(start_before) = filters.start_before {
            query_builder.push(" AND start_date <= ");
            query_builder.push_bind(start_before);
        }

        if let Some(sort_field) = sort_by {
            query_builder.push(format!(" ORDER BY {sort_field} {sort_order}"));
        } else {
            query_builder.push(" ORDER BY created_at DESC");
        }

        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        let results = query_builder.build().fetch_all(&self.pool).await?;

        Ok(results
            .into_iter()
            .map(|r| Budget {
                id: r.get("id"),
                user_id: r.get("user_id"),
                category: r.get("category"),
                amount: r.get("amount"),
                period: serde_json::from_str(r.get("period")).unwrap_or(BudgetPeriod::Monthly),
                start_date: r.get("start_date"),
                end_date: r.get("end_date"),
                is_active: r.get("is_active"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .collect())
    }

    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::BudgetFilters,
    ) -> Result<i64> {
        let mut query_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM budgets WHERE 1=1");

        if let Some(user_id) = filters.user_id {
            query_builder.push(" AND user_id = ");
            query_builder.push_bind(user_id);
        }

        if let Some(ref category) = filters.category {
            query_builder.push(" AND category = ");
            query_builder.push_bind(category);
        }

        if let Some(ref period) = filters.period {
            let period_str = serde_json::to_string(period).unwrap_or_default();
            query_builder.push(" AND period = ");
            query_builder.push_bind(period_str);
        }

        if let Some(is_active) = filters.is_active {
            query_builder.push(" AND is_active = ");
            query_builder.push_bind(is_active);
        }

        if let Some(min_amount) = filters.min_amount {
            query_builder.push(" AND amount >= ");
            query_builder.push_bind(min_amount);
        }

        if let Some(max_amount) = filters.max_amount {
            query_builder.push(" AND amount <= ");
            query_builder.push_bind(max_amount);
        }

        if let Some(start_after) = filters.start_after {
            query_builder.push(" AND start_date >= ");
            query_builder.push_bind(start_after);
        }

        if let Some(start_before) = filters.start_before {
            query_builder.push(" AND start_date <= ");
            query_builder.push_bind(start_before);
        }

        let result: (i64,) = query_builder.build_query_as().fetch_one(&self.pool).await?;

        Ok(result.0)
    }

    async fn update(&self, budget: Budget) -> Result<Budget> {
        let period_str = serde_json::to_string(&budget.period)?;
        let result = sqlx::query("UPDATE budgets SET category=$2,amount=$3,period=$4,start_date=$5,end_date=$6,is_active=$7,updated_at=$8 WHERE id=$1 RETURNING *")
      .bind(budget.id).bind(&budget.category).bind(budget.amount).bind(period_str).bind(budget.start_date).bind(budget.end_date).bind(budget.is_active).bind(budget.updated_at).fetch_one(&self.pool).await?;
        Ok(Budget {
            id: result.get("id"),
            user_id: result.get("user_id"),
            category: result.get("category"),
            amount: result.get("amount"),
            period: serde_json::from_str(result.get("period"))?,
            start_date: result.get("start_date"),
            end_date: result.get("end_date"),
            is_active: result.get("is_active"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        })
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM budgets WHERE id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
