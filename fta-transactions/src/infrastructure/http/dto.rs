use chrono::{DateTime, Utc};
use fta_validation::{prelude::*, ObjectSchema, Validatable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::{Transaction, TransactionType};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
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

impl From<Transaction> for TransactionResponse {
    fn from(transaction: Transaction) -> Self {
        Self {
            id: transaction.id,
            account_id: transaction.account_id,
            transaction_type: transaction.transaction_type,
            amount: transaction.amount,
            category: transaction.category,
            description: transaction.description,
            transaction_date: transaction.transaction_date,
            created_at: transaction.created_at,
            updated_at: transaction.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTransactionRequest {
    pub account_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub category: Option<String>,
    pub description: Option<String>,
    pub transaction_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateTransactionRequest {
    pub category: Option<String>,
    pub description: Option<String>,
}

impl Validatable for CreateTransactionRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("account_id", string())
            .field("transaction_type", string())
            .field("amount", number().positive().int())
            .field("category", string().min(1).max(50).optional())
            .field("description", string().max(500).optional())
            .field("transaction_date", string())
    }
}

impl Validatable for UpdateTransactionRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("category", string().min(1).max(50).optional())
            .field("description", string().max(500).optional())
    }
}
