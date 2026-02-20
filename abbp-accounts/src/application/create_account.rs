//! Create account use case.

use crate::domain::{Account, AccountRepository, AccountType};
use abbp_errors::AppError;
use abbp_types::impl_use_case_debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

/// Creates a new financial account for a user.
pub struct CreateAccount {
    repository: Arc<dyn AccountRepository>,
}

impl_use_case_debug!(CreateAccount);

impl CreateAccount {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    /// Creates a new account with the given parameters.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user who owns the account
    /// * `name` - The name of the account
    /// * `account_type` - The type of account (Checking, Savings, etc.)
    /// * `initial_balance` - The starting balance in cents
    /// * `currency` - ISO 4217 currency code (e.g., "USD")
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn execute(
        &self,
        user_id: Uuid,
        name: String,
        account_type: AccountType,
        initial_balance: i64,
        currency: String,
    ) -> Result<Account, AppError> {
        tracing::debug!("Creating new account");
        let account = Account::new(user_id, name, account_type, initial_balance, currency);
        let result = self.repository.create(account).await?;
        tracing::info!(account_id = %result.id, "Account created successfully");
        Ok(result)
    }
}
