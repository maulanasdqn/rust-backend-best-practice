use std::sync::Arc;

use abbp_accounts::PostgresAccountRepository;
use abbp_auth::{
    PostgresEmailVerificationRepository, PostgresPasswordResetTokenRepository,
    PostgresRefreshTokenRepository,
};
use abbp_budgets::PostgresBudgetRepository;
use abbp_transactions::PostgresTransactionRepository;
use abbp_users::PostgresUserRepository;

pub type Repositories = (
    Arc<PostgresUserRepository>,
    Arc<PostgresAccountRepository>,
    Arc<PostgresTransactionRepository>,
    Arc<PostgresBudgetRepository>,
    Arc<PostgresRefreshTokenRepository>,
    Arc<PostgresEmailVerificationRepository>,
    Arc<PostgresPasswordResetTokenRepository>,
);

pub fn setup_repositories(db_pool: abbp_database::DbPool) -> Repositories {
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
