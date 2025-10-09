pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types
pub use application::{
    CreateAccount, DeactivateAccount, DeleteAccount, GetAccount, ListAccounts, UpdateAccount,
};
pub use domain::{Account, AccountRepository, AccountType};
pub use infrastructure::{
    account_routes, AccountResponse, CreateAccountRequest, PostgresAccountRepository,
    UpdateAccountRequest,
};
