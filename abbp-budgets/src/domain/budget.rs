use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum BudgetPeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub amount: i64,
    pub period: BudgetPeriod,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Budget {
    pub fn new(
        user_id: Uuid,
        category: String,
        amount: i64,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            category,
            amount,
            period,
            start_date,
            end_date: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_amount(&mut self, new_amount: i64) {
        self.amount = new_amount;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self, end_date: DateTime<Utc>) {
        self.is_active = false;
        self.end_date = Some(end_date);
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.end_date = None;
        self.updated_at = Utc::now();
    }
}
