#![allow(clippy::inconsistent_digit_grouping)]

use fta_accounts::domain::{Account, AccountType};
use uuid::Uuid;

#[test]
fn test_update_balance_adds_positive_amount() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    account.update_balance(5000);
    assert_eq!(account.balance, 15000);
}

#[test]
fn test_update_balance_adds_negative_amount() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    account.update_balance(-3000);
    assert_eq!(account.balance, 7000);
}

#[test]
fn test_update_balance_can_result_in_negative_balance() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Credit Card".to_string(),
        AccountType::Credit,
        0,
        "USD".to_string(),
    );

    account.update_balance(-10000);
    assert_eq!(account.balance, -10000);
}

#[test]
fn test_update_balance_updates_timestamp() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    let original_updated_at = account.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(10));
    account.update_balance(100);

    assert!(account.updated_at > original_updated_at);
}

#[test]
fn test_multiple_balance_updates() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    account.update_balance(2000);
    account.update_balance(-500);
    account.update_balance(1000);

    assert_eq!(account.balance, 12500);
}

#[test]
fn test_account_with_max_i64_balance() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Max Balance".to_string(),
        AccountType::Investment,
        i64::MAX,
        "USD".to_string(),
    );

    assert_eq!(account.balance, i64::MAX);
}

#[test]
fn test_account_with_min_i64_balance() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Min Balance".to_string(),
        AccountType::Credit,
        i64::MIN,
        "USD".to_string(),
    );

    assert_eq!(account.balance, i64::MIN);
}

#[test]
fn test_balance_update_with_max_value() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        0,
        "USD".to_string(),
    );

    account.update_balance(i64::MAX);
    assert_eq!(account.balance, i64::MAX);
}

#[test]
fn test_balance_update_with_min_value() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Credit,
        0,
        "USD".to_string(),
    );

    account.update_balance(i64::MIN);
    assert_eq!(account.balance, i64::MIN);
}

#[test]
fn test_very_large_positive_balance() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Billionaire Account".to_string(),
        AccountType::Investment,
        100_000_000_000_00,
        "USD".to_string(),
    );

    account.update_balance(50_000_000_000_00);
    assert_eq!(account.balance, 150_000_000_000_00);
}

#[test]
fn test_very_large_negative_balance() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Debt Account".to_string(),
        AccountType::Credit,
        -100_000_000_00,
        "USD".to_string(),
    );

    account.update_balance(-50_000_000_00);
    assert_eq!(account.balance, -150_000_000_00);
}
