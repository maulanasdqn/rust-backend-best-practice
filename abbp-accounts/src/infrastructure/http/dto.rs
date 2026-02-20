use chrono::{DateTime, Utc};
use abbp_validation::{prelude::*, ObjectSchema, Validatable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::{Account, AccountType};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub balance: i64,
    pub currency: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            user_id: account.user_id,
            name: account.name,
            account_type: account.account_type,
            balance: account.balance,
            currency: account.currency,
            is_active: account.is_active,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateAccountRequest {
    pub name: String,
    pub account_type: AccountType,
    pub initial_balance: i64,
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
}

impl Validatable for CreateAccountRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("name", string().min(1).max(100))
            .field("account_type", string())
            .field("initial_balance", number().int())
            .field("currency", string().min(3).max(3).regex(r"^[A-Z]{3}$"))
    }
}

impl Validatable for UpdateAccountRequest {
    fn schema() -> ObjectSchema {
        object().field("name", string().min(1).max(100).optional())
    }
}
