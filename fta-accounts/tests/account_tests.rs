#![allow(clippy::inconsistent_digit_grouping)]

use fta_accounts::domain::{Account, AccountType};
use uuid::Uuid;

#[allow(dead_code)]
fn test_new_creates_account_with_correct_fields() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Checking Account".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    assert_eq!(account.user_id, user_id);
    assert_eq!(account.name, "Checking Account");
    assert_eq!(account.account_type, AccountType::Checking);
    assert_eq!(account.balance, 10000);
    assert_eq!(account.currency, "USD");
    assert!(account.is_active);
}

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
fn test_deactivate_sets_is_active_to_false() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    assert!(account.is_active);
    account.deactivate();
    assert!(!account.is_active);
}

#[test]
fn test_deactivate_updates_timestamp() {
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
    account.deactivate();

    assert!(account.updated_at > original_updated_at);
}

#[test]
fn test_activate_sets_is_active_to_true() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    account.deactivate();
    assert!(!account.is_active);

    account.activate();
    assert!(account.is_active);
}

#[test]
fn test_activate_updates_timestamp() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    account.deactivate();
    let deactivated_updated_at = account.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(10));
    account.activate();

    assert!(account.updated_at > deactivated_updated_at);
}

#[test]
fn test_rename_changes_account_name() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Old Name".to_string(),
        AccountType::Checking,
        10000,
        "USD".to_string(),
    );

    account.rename("New Name".to_string());
    assert_eq!(account.name, "New Name");
}

#[test]
fn test_rename_updates_timestamp() {
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
    account.rename("New Name".to_string());

    assert!(account.updated_at > original_updated_at);
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

// Edge case tests
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
fn test_account_with_empty_name() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        String::new(),
        AccountType::Checking,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.name, "");
}

#[test]
fn test_account_with_very_long_name() {
    let user_id = Uuid::new_v4();
    let long_name = "A".repeat(1000);
    let account = Account::new(
        user_id,
        long_name.clone(),
        AccountType::Checking,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.name, long_name);
}

#[test]
fn test_account_with_unicode_name() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "账户名称 🏦 💰".to_string(),
        AccountType::Checking,
        10000,
        "CNY".to_string(),
    );

    assert_eq!(account.name, "账户名称 🏦 💰");
}

#[test]
fn test_account_with_special_characters_in_name() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Test!@#$%^&*()_+-=[]{}|;:',.<>?/~`".to_string(),
        AccountType::Checking,
        0,
        "USD".to_string(),
    );

    assert_eq!(account.name, "Test!@#$%^&*()_+-=[]{}|;:',.<>?/~`");
}

#[test]
fn test_account_with_empty_currency() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        0,
        String::new(),
    );

    assert_eq!(account.currency, "");
}

#[test]
fn test_account_with_non_standard_currency() {
    let user_id = Uuid::new_v4();
    let account = Account::new(
        user_id,
        "Bitcoin Account".to_string(),
        AccountType::Investment,
        0,
        "BTC".to_string(),
    );

    assert_eq!(account.currency, "BTC");
}

#[test]
fn test_rename_to_empty_string() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Original Name".to_string(),
        AccountType::Checking,
        0,
        "USD".to_string(),
    );

    account.rename(String::new());
    assert_eq!(account.name, "");
}

#[test]
fn test_rename_to_very_long_name() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Short".to_string(),
        AccountType::Checking,
        0,
        "USD".to_string(),
    );

    let long_name = "B".repeat(5000);
    account.rename(long_name.clone());
    assert_eq!(account.name, long_name);
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
fn test_multiple_activations_deactivations() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Test".to_string(),
        AccountType::Checking,
        0,
        "USD".to_string(),
    );

    for _ in 0..10 {
        account.deactivate();
        assert!(!account.is_active);
        account.activate();
        assert!(account.is_active);
    }
}

#[test]
fn test_very_large_positive_balance() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Billionaire Account".to_string(),
        AccountType::Investment,
        100_000_000_000_00, // $1 billion
        "USD".to_string(),
    );

    account.update_balance(50_000_000_000_00); // Add $500 million
    assert_eq!(account.balance, 150_000_000_000_00); // $1.5 billion
}

#[test]
fn test_very_large_negative_balance() {
    let user_id = Uuid::new_v4();
    let mut account = Account::new(
        user_id,
        "Debt Account".to_string(),
        AccountType::Credit,
        -100_000_000_00, // -$1 million in debt
        "USD".to_string(),
    );

    account.update_balance(-50_000_000_00); // More debt
    assert_eq!(account.balance, -150_000_000_00);
}
