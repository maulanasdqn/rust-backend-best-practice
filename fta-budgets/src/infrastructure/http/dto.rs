use crate::domain::{Budget, BudgetPeriod};
use chrono::{DateTime, Utc};
use fta_validation::{prelude::*, ObjectSchema, Validatable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BudgetResponse {
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

impl From<Budget> for BudgetResponse {
    fn from(b: Budget) -> Self {
        Self {
            id: b.id,
            user_id: b.user_id,
            category: b.category,
            amount: b.amount,
            period: b.period,
            start_date: b.start_date,
            end_date: b.end_date,
            is_active: b.is_active,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateBudgetRequest {
    pub category: String,
    pub amount: i64,
    pub period: BudgetPeriod,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateBudgetRequest {
    pub category: Option<String>,
    pub amount: Option<i64>,
}

impl Validatable for CreateBudgetRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("category", string().min(1).max(100))
            .field("amount", number().positive().int())
            .field("period", string())
            .field("start_date", string())
            .field("end_date", string().optional())
    }
}

impl Validatable for UpdateBudgetRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("category", string().min(1).max(100).optional())
            .field("amount", number().positive().int().optional())
    }
}
