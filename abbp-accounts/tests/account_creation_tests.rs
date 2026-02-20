use abbp_accounts::domain::{Account, AccountType};
use uuid::Uuid;

#[test]
fn test_new_account_is_active_by_default() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Savings".to_string(),
        AccountType::Savings,
        0,
        "USD".to_string(),
    );

    assert!(account.is_active);
}

#[test]
fn test_new_account_timestamps_are_set() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Cash,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.created_at, account.updated_at);
}

#[test]
fn test_account_type_checking() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.account_type, AccountType::Checking);
}

#[test]
fn test_account_type_savings() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Savings,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.account_type, AccountType::Savings);
}

#[test]
fn test_account_type_credit() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Credit,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.account_type, AccountType::Credit);
}

#[test]
fn test_account_type_investment() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Investment,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.account_type, AccountType::Investment);
}

#[test]
fn test_account_type_cash() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Cash,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.account_type, AccountType::Cash);
}
