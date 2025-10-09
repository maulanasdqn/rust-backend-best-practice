use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
    Investment,
    Cash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Account {
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

impl Account {
    pub fn new(
        user_id: Uuid,
        name: String,
        account_type: AccountType,
        initial_balance: i64,
        currency: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            account_type,
            balance: initial_balance,
            currency,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_balance(&mut self, amount: i64) {
        self.balance += amount;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
        self.updated_at = Utc::now();
    }
}
