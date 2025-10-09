use fta_errors::AppError;
use paginator_rs::PaginationParams;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Transaction, TransactionRepository};

pub struct ListTransactions {
    repository: Arc<dyn TransactionRepository>,
}

impl std::fmt::Debug for ListTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListTransactions").finish()
    }
}

impl ListTransactions {
    pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        params: &PaginationParams,
    ) -> Result<(Vec<Transaction>, i64), AppError> {
        let total = self
            .repository
            .count_by_account_id(&account_id)
            .await
            .map_err(AppError::from)?;
        let transactions = self
            .repository
            .find_by_account_id(
                &account_id,
                i64::from(params.limit()),
                i64::from(params.offset()),
            )
            .await
            .map_err(AppError::from)?;
        Ok((transactions, total))
    }
}
