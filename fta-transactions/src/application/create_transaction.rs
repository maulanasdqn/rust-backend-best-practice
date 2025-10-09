use chrono::{DateTime, Utc};
use fta_errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Transaction, TransactionRepository, TransactionType};

pub struct CreateTransaction {
    repository: Arc<dyn TransactionRepository>,
}

impl std::fmt::Debug for CreateTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateTransaction").finish()
    }
}

impl CreateTransaction {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        transaction_type: TransactionType,
        amount: i64,
        category: Option<String>,
        description: Option<String>,
        transaction_date: DateTime<Utc>,
    ) -> Result<Transaction, AppError> {
        let transaction = Transaction::new(
            account_id,
            transaction_type,
            amount,
            category,
            description,
            transaction_date,
        );

        self.repository
            .create(transaction)
            .await
            .map_err(AppError::from)
    }
}
