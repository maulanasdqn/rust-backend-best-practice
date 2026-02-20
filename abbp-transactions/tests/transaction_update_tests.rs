use chrono::Utc;
use abbp_transactions::domain::{Transaction, TransactionType};
use uuid::Uuid;

#[test]
fn test_update_details_changes_both_fields() {
    let account_id = Uuid::new_v4();
    let mut transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        Some("Old Category".to_string()),
        Some("Old Description".to_string()),
        Utc::now(),
    );

    transaction.update_details(
        Some("New Category".to_string()),
        Some("New Description".to_string()),
    );

    assert_eq!(transaction.category, Some("New Category".to_string()));
    assert_eq!(transaction.description, Some("New Description".to_string()));
}

#[test]
fn test_update_details_can_set_to_none() {
    let account_id = Uuid::new_v4();
    let mut transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        Some("Category".to_string()),
        Some("Description".to_string()),
        Utc::now(),
    );

    transaction.update_details(None, None);

    assert!(transaction.category.is_none());
    assert!(transaction.description.is_none());
}

#[test]
fn test_update_details_can_update_category_only() {
    let account_id = Uuid::new_v4();
    let mut transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        Some("Old Category".to_string()),
        Some("Description".to_string()),
        Utc::now(),
    );

    transaction.update_details(
        Some("New Category".to_string()),
        Some("Description".to_string()),
    );

    assert_eq!(transaction.category, Some("New Category".to_string()));
    assert_eq!(transaction.description, Some("Description".to_string()));
}

#[test]
fn test_update_details_can_update_description_only() {
    let account_id = Uuid::new_v4();
    let mut transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        Some("Category".to_string()),
        Some("Old Description".to_string()),
        Utc::now(),
    );

    transaction.update_details(
        Some("Category".to_string()),
        Some("New Description".to_string()),
    );

    assert_eq!(transaction.category, Some("Category".to_string()));
    assert_eq!(transaction.description, Some("New Description".to_string()));
}

#[test]
fn test_update_details_updates_timestamp() {
    let account_id = Uuid::new_v4();
    let mut transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        None,
        None,
        Utc::now(),
    );

    let original_updated_at = transaction.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(10));
    transaction.update_details(Some("Category".to_string()), None);

    assert!(transaction.updated_at > original_updated_at);
}

#[test]
fn test_multiple_detail_updates() {
    let account_id = Uuid::new_v4();
    let mut transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        None,
        None,
        Utc::now(),
    );

    transaction.update_details(Some("Category 1".to_string()), Some("Desc 1".to_string()));
    transaction.update_details(Some("Category 2".to_string()), Some("Desc 2".to_string()));
    transaction.update_details(Some("Category 3".to_string()), Some("Desc 3".to_string()));

    assert_eq!(transaction.category, Some("Category 3".to_string()));
    assert_eq!(transaction.description, Some("Desc 3".to_string()));
}

#[test]
fn test_update_details_to_empty_strings() {
    let account_id = Uuid::new_v4();
    let mut transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        Some("Original".to_string()),
        Some("Original Desc".to_string()),
        Utc::now(),
    );

    transaction.update_details(Some(String::new()), Some(String::new()));

    assert_eq!(transaction.category, Some(String::new()));
    assert_eq!(transaction.description, Some(String::new()));
}
