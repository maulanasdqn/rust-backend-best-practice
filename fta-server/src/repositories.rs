use std::sync::Arc;

use fta_accounts::PostgresAccountRepository;
use fta_auth::{
    PostgresEmailVerificationRepository, PostgresPasswordResetTokenRepository,
    PostgresRefreshTokenRepository,
};
use fta_budgets::PostgresBudgetRepository;
use fta_transactions::PostgresTransactionRepository;
use fta_users::PostgresUserRepository;

pub type Repositories = (
    Arc<PostgresUserRepository>,
    Arc<PostgresAccountRepository>,
    Arc<PostgresTransactionRepository>,
    Arc<PostgresBudgetRepository>,
    Arc<PostgresRefreshTokenRepository>,
    Arc<PostgresEmailVerificationRepository>,
    Arc<PostgresPasswordResetTokenRepository>,
);

pub fn setup_repositories(db_pool: fta_database::DbPool) -> Repositories {
    let user_repository = Arc::new(PostgresUserRepository::new(db_pool.clone()));
    let account_repository = Arc::new(PostgresAccountRepository::new(db_pool.clone()));
    let transaction_repository = Arc::new(PostgresTransactionRepository::new(db_pool.clone()));
    let budget_repository = Arc::new(PostgresBudgetRepository::new(db_pool.clone()));
    let refresh_token_repository = Arc::new(PostgresRefreshTokenRepository::new(db_pool.clone()));
    let email_verification_repository =
        Arc::new(PostgresEmailVerificationRepository::new(db_pool.clone()));
    let password_reset_token_repository =
        Arc::new(PostgresPasswordResetTokenRepository::new(db_pool));

    (
        user_repository,
        account_repository,
        transaction_repository,
        budget_repository,
        refresh_token_repository,
        email_verification_repository,
        password_reset_token_repository,
    )
}
