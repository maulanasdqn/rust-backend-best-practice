use abbp_accounts::domain::{Account, AccountType};
use uuid::Uuid;

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
