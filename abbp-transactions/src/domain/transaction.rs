use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Income,
    Expense,
    Transfer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub category: Option<String>,
    pub description: Option<String>,
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        account_id: Uuid,
        transaction_type: TransactionType,
        amount: i64,
        category: Option<String>,
        description: Option<String>,
        transaction_date: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            account_id,
            transaction_type,
            amount,
            category,
            description,
            transaction_date,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_details(&mut self, category: Option<String>, description: Option<String>) {
        self.category = category;
        self.description = description;
        self.updated_at = Utc::now();
    }
}
