use anyhow::Result;
use async_trait::async_trait;
use fta_database::{entities::accounts, sea_orm, DbPool};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::domain::{Account, AccountRepository, AccountType};
use crate::infrastructure::http::filters::AccountFilters;

fn model_to_account(model: accounts::Model) -> Account {
    let account_type: AccountType =
        serde_json::from_str(&model.account_type).unwrap_or(AccountType::Checking);

    Account {
        id: model.id,
        user_id: model.user_id,
        name: model.name,
        account_type,
        balance: model.balance,
        currency: model.currency,
        is_active: model.is_active,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

#[derive(Clone, Debug)]
pub struct PostgresAccountRepository {
    pool: DbPool,
}

impl PostgresAccountRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for PostgresAccountRepository {
    async fn create(&self, account: Account) -> Result<Account> {
        let account_type_str = serde_json::to_string(&account.account_type)?;

        let active_model = accounts::ActiveModel {
            id: Set(account.id),
            user_id: Set(account.user_id),
            name: Set(account.name.clone()),
            account_type: Set(account_type_str),
            balance: Set(account.balance),
            currency: Set(account.currency.clone()),
            is_active: Set(account.is_active),
            created_at: Set(account.created_at),
            updated_at: Set(account.updated_at),
        };

        let result = active_model.insert(&self.pool).await?;
        Ok(model_to_account(result))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Account>> {
        let result = accounts::Entity::find_by_id(*id).one(&self.pool).await?;
        Ok(result.map(model_to_account))
    }

    async fn find_all(
        &self,
        filters: &AccountFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Account>> {
        let mut query = accounts::Entity::find();

        // Apply filters
        if let Some(user_id) = filters.user_id {
            query = query.filter(accounts::Column::UserId.eq(user_id));
        }
        if let Some(ref account_type) = filters.account_type {
            let account_type_str = serde_json::to_string(account_type).unwrap_or_default();
            query = query.filter(accounts::Column::AccountType.eq(account_type_str));
        }
        if let Some(ref currency) = filters.currency {
            query = query.filter(accounts::Column::Currency.eq(currency));
        }
        if let Some(is_active) = filters.is_active {
            query = query.filter(accounts::Column::IsActive.eq(is_active));
        }
        if let Some(min_balance) = filters.min_balance {
            query = query.filter(accounts::Column::Balance.gte(min_balance));
        }
        if let Some(max_balance) = filters.max_balance {
            query = query.filter(accounts::Column::Balance.lte(max_balance));
        }
        if let Some(ref name) = filters.name {
            query = query.filter(accounts::Column::Name.contains(name));
        }

        // Apply sorting
        let order = if sort_order.to_lowercase() == "asc" {
            sea_orm::Order::Asc
        } else {
            sea_orm::Order::Desc
        };

        query = match sort_by {
            Some("name") => query.order_by(accounts::Column::Name, order),
            Some("balance") => query.order_by(accounts::Column::Balance, order),
            Some("created_at") | None => query.order_by(accounts::Column::CreatedAt, order),
            Some("updated_at") => query.order_by(accounts::Column::UpdatedAt, order),
            Some(_) => query.order_by(accounts::Column::CreatedAt, order),
        };

        let results = query
            .paginate(&self.pool, limit as u64)
            .fetch_page((offset / limit.max(1)) as u64)
            .await?;

        Ok(results.into_iter().map(model_to_account).collect())
    }

    async fn count_all(&self, filters: &AccountFilters) -> Result<i64> {
        let mut query = accounts::Entity::find();

        // Apply filters
        if let Some(user_id) = filters.user_id {
            query = query.filter(accounts::Column::UserId.eq(user_id));
        }
        if let Some(ref account_type) = filters.account_type {
            let account_type_str = serde_json::to_string(account_type).unwrap_or_default();
            query = query.filter(accounts::Column::AccountType.eq(account_type_str));
        }
        if let Some(ref currency) = filters.currency {
            query = query.filter(accounts::Column::Currency.eq(currency));
        }
        if let Some(is_active) = filters.is_active {
            query = query.filter(accounts::Column::IsActive.eq(is_active));
        }
        if let Some(min_balance) = filters.min_balance {
            query = query.filter(accounts::Column::Balance.gte(min_balance));
        }
        if let Some(max_balance) = filters.max_balance {
            query = query.filter(accounts::Column::Balance.lte(max_balance));
        }
        if let Some(ref name) = filters.name {
            query = query.filter(accounts::Column::Name.contains(name));
        }

        let count = query.count(&self.pool).await?;
        Ok(count as i64)
    }

    async fn update(&self, account: Account) -> Result<Account> {
        let account_type_str = serde_json::to_string(&account.account_type)?;

        let active_model = accounts::ActiveModel {
            id: Set(account.id),
            user_id: Set(account.user_id),
            name: Set(account.name.clone()),
            account_type: Set(account_type_str),
            balance: Set(account.balance),
            currency: Set(account.currency.clone()),
            is_active: Set(account.is_active),
            created_at: Set(account.created_at),
            updated_at: Set(account.updated_at),
        };

        let result = active_model.update(&self.pool).await?;
        Ok(model_to_account(result))
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        accounts::Entity::delete_by_id(*id).exec(&self.pool).await?;
        Ok(())
    }
}
