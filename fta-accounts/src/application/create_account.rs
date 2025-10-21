use crate::domain::{Account, AccountRepository, AccountType};
use fta_errors::AppError;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateAccount {
    repository: Arc<dyn AccountRepository>,
}

impl Debug for CreateAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateAccount").finish()
    }
}

impl CreateAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        name: String,
        account_type: AccountType,
        initial_balance: i64,
        currency: String,
    ) -> Result<Account, AppError> {
        let account = Account::new(user_id, name, account_type, initial_balance, currency);

        self.repository
            .create(account)
            .await
            .map_err(AppError::from)
    }
}
