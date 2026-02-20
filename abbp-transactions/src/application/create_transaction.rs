//! Create transaction use case.

use chrono::{DateTime, Utc};
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::{Transaction, TransactionRepository, TransactionType};

/// Creates a new financial transaction.
pub struct CreateTransaction {
    repository: Arc<dyn TransactionRepository>,
}

impl_use_case_debug!(CreateTransaction);

impl CreateTransaction {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    /// Creates a new transaction for an account.
    #[instrument(skip(self), fields(account_id = %account_id, amount = amount))]
    pub async fn execute(
        &self,
        account_id: Uuid,
        transaction_type: TransactionType,
        amount: i64,
        category: Option<String>,
        description: Option<String>,
        transaction_date: DateTime<Utc>,
    ) -> Result<Transaction, AppError> {
        tracing::debug!("Creating new transaction");
        let transaction = Transaction::new(
            account_id,
            transaction_type,
            amount,
            category,
            description,
            transaction_date,
        );

        let result = self.repository.create(transaction).await?;
        tracing::info!(transaction_id = %result.id, "Transaction created successfully");
        Ok(result)
    }
}
