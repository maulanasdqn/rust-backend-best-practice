use std::sync::Arc;

use abbp_accounts::{
    CreateAccount, DeactivateAccount, DeleteAccount, GetAccount, ListAccounts,
    PostgresAccountRepository, UpdateAccount,
};
use abbp_budgets::{
    CreateBudget, DeactivateBudget, DeleteBudget, GetBudget, ListBudgets, PostgresBudgetRepository,
    UpdateBudget,
};
use abbp_transactions::{
    CreateTransaction, DeleteTransaction, GetTransaction, ListTransactions,
    PostgresTransactionRepository, UpdateTransaction,
};
use abbp_users::{CreateUser, DeleteUser, GetUser, ListUsers, UpdateUser};

pub struct UseCases {
    pub get_user: Arc<GetUser>,
    pub list_users: Arc<ListUsers>,
    pub create_user: Arc<CreateUser>,
    pub update_user: Arc<UpdateUser>,
    pub delete_user: Arc<DeleteUser>,
    pub get_account: Arc<GetAccount>,
    pub list_accounts: Arc<ListAccounts>,
    pub create_account: Arc<CreateAccount>,
    pub update_account: Arc<UpdateAccount>,
    pub delete_account: Arc<DeleteAccount>,
    pub deactivate_account: Arc<DeactivateAccount>,
    pub get_transaction: Arc<GetTransaction>,
    pub list_transactions: Arc<ListTransactions>,
    pub create_transaction: Arc<CreateTransaction>,
    pub update_transaction: Arc<UpdateTransaction>,
    pub delete_transaction: Arc<DeleteTransaction>,
    pub get_budget: Arc<GetBudget>,
    pub list_budgets: Arc<ListBudgets>,
    pub create_budget: Arc<CreateBudget>,
    pub update_budget: Arc<UpdateBudget>,
    pub delete_budget: Arc<DeleteBudget>,
    pub deactivate_budget: Arc<DeactivateBudget>,
}

pub fn init_use_cases(
    user_repository: Arc<dyn abbp_users::domain::UserRepository>,
    account_repository: Arc<PostgresAccountRepository>,
    transaction_repository: Arc<PostgresTransactionRepository>,
    budget_repository: Arc<PostgresBudgetRepository>,
) -> UseCases {
    let get_user = Arc::new(GetUser::new(user_repository.clone()));
    let list_users = Arc::new(ListUsers::new(user_repository.clone()));
    let create_user = Arc::new(CreateUser::new(user_repository.clone()));
    let update_user = Arc::new(UpdateUser::new(user_repository.clone()));
    let delete_user = Arc::new(DeleteUser::new(user_repository));

    let get_account = Arc::new(GetAccount::new(account_repository.clone()));
    let list_accounts = Arc::new(ListAccounts::new(account_repository.clone()));
    let create_account = Arc::new(CreateAccount::new(account_repository.clone()));
    let update_account = Arc::new(UpdateAccount::new(account_repository.clone()));
    let delete_account = Arc::new(DeleteAccount::new(account_repository.clone()));
    let deactivate_account = Arc::new(DeactivateAccount::new(account_repository));

    let get_transaction = Arc::new(GetTransaction::new(transaction_repository.clone()));
    let list_transactions = Arc::new(ListTransactions::new(transaction_repository.clone()));
    let create_transaction = Arc::new(CreateTransaction::new(transaction_repository.clone()));
    let update_transaction = Arc::new(UpdateTransaction::new(transaction_repository.clone()));
    let delete_transaction = Arc::new(DeleteTransaction::new(transaction_repository));

    let get_budget = Arc::new(GetBudget::new(budget_repository.clone()));
    let list_budgets = Arc::new(ListBudgets::new(budget_repository.clone()));
    let create_budget = Arc::new(CreateBudget::new(budget_repository.clone()));
    let update_budget = Arc::new(UpdateBudget::new(budget_repository.clone()));
    let delete_budget = Arc::new(DeleteBudget::new(budget_repository.clone()));
    let deactivate_budget = Arc::new(DeactivateBudget::new(budget_repository));

    UseCases {
        get_user,
        list_users,
        create_user,
        update_user,
        delete_user,
        get_account,
        list_accounts,
        create_account,
        update_account,
        delete_account,
        deactivate_account,
        get_transaction,
        list_transactions,
        create_transaction,
        update_transaction,
        delete_transaction,
        get_budget,
        list_budgets,
        create_budget,
        update_budget,
        delete_budget,
        deactivate_budget,
    }
}
