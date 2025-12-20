use fta_accounts::domain::{Account, AccountType};
use uuid::Uuid;

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
