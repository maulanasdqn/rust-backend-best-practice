#![allow(clippy::inconsistent_digit_grouping)]

use chrono::{DateTime, Utc};
use fta_transactions::domain::{Transaction, TransactionType};
use uuid::Uuid;

#[allow(dead_code)]
fn test_new_creates_transaction_with_correct_fields() {
    let account_id = Uuid::new_v4();
    let transaction_date = Utc::now();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Income,
        10000,
        Some("Salary".to_string()),
        Some("Monthly salary".to_string()),
        transaction_date,
    );

    assert_eq!(transaction.account_id, account_id);
    assert_eq!(transaction.transaction_type, TransactionType::Income);
    assert_eq!(transaction.amount, 10000);
    assert_eq!(transaction.category, Some("Salary".to_string()));
    assert_eq!(transaction.description, Some("Monthly salary".to_string()));
    assert_eq!(transaction.transaction_date, transaction_date);
}

#[test]
fn test_new_transaction_with_none_category_and_description() {
    let account_id = Uuid::new_v4();
    let transaction_date = Utc::now();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        None,
        None,
        transaction_date,
    );

    assert!(transaction.category.is_none());
    assert!(transaction.description.is_none());
}

#[test]
fn test_new_transaction_timestamps_are_set() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.created_at, transaction.updated_at);
}

#[test]
fn test_transaction_type_income() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Income,
        10000,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.transaction_type, TransactionType::Income);
}

#[test]
fn test_transaction_type_expense() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.transaction_type, TransactionType::Expense);
}

#[test]
fn test_transaction_type_transfer() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Transfer,
        7500,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.transaction_type, TransactionType::Transfer);
}

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
fn test_transaction_with_positive_amount() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Income,
        10000,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.amount, 10000);
}

#[test]
fn test_transaction_with_zero_amount() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        0,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.amount, 0);
}

#[test]
fn test_transaction_date_is_preserved() {
    let account_id = Uuid::new_v4();
    let specific_date = DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        5000,
        None,
        None,
        specific_date,
    );

    assert_eq!(transaction.transaction_date, specific_date);
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

// Edge case tests
#[test]
fn test_transaction_with_max_i64_amount() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Income,
        i64::MAX,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.amount, i64::MAX);
}

#[test]
fn test_transaction_with_min_i64_amount() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        i64::MIN,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.amount, i64::MIN);
}

#[test]
fn test_transaction_with_empty_category() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        Some(String::new()),
        None,
        Utc::now(),
    );

    assert_eq!(transaction.category, Some(String::new()));
}

#[test]
fn test_transaction_with_empty_description() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        None,
        Some(String::new()),
        Utc::now(),
    );

    assert_eq!(transaction.description, Some(String::new()));
}

#[test]
fn test_transaction_with_very_long_category() {
    let account_id = Uuid::new_v4();
    let long_category = "C".repeat(10000);
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        Some(long_category.clone()),
        None,
        Utc::now(),
    );

    assert_eq!(transaction.category, Some(long_category));
}

#[test]
fn test_transaction_with_very_long_description() {
    let account_id = Uuid::new_v4();
    let long_description = "D".repeat(50000);
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        None,
        Some(long_description.clone()),
        Utc::now(),
    );

    assert_eq!(transaction.description, Some(long_description));
}

#[test]
fn test_transaction_with_unicode_category() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        Some("食品 🍕 🍔".to_string()),
        None,
        Utc::now(),
    );

    assert_eq!(transaction.category, Some("食品 🍕 🍔".to_string()));
}

#[test]
fn test_transaction_with_unicode_description() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Income,
        5000,
        None,
        Some("工资支付 💰 ¥".to_string()),
        Utc::now(),
    );

    assert_eq!(transaction.description, Some("工资支付 💰 ¥".to_string()));
}

#[test]
fn test_transaction_with_special_characters() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        Some("Test!@#$%^&*()".to_string()),
        Some("Description<>{}[]|\\".to_string()),
        Utc::now(),
    );

    assert_eq!(transaction.category, Some("Test!@#$%^&*()".to_string()));
    assert_eq!(
        transaction.description,
        Some("Description<>{}[]|\\".to_string())
    );
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

#[test]
fn test_transaction_with_very_old_date() {
    let account_id = Uuid::new_v4();
    let old_date = DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        1000,
        None,
        None,
        old_date,
    );

    assert_eq!(transaction.transaction_date, old_date);
}

#[test]
fn test_transaction_with_far_future_date() {
    let account_id = Uuid::new_v4();
    let future_date = DateTime::parse_from_rfc3339("2099-12-31T23:59:59Z")
        .unwrap()
        .with_timezone(&Utc);

    let transaction = Transaction::new(
        account_id,
        TransactionType::Income,
        1000,
        None,
        None,
        future_date,
    );

    assert_eq!(transaction.transaction_date, future_date);
}

#[test]
fn test_very_large_transaction_amount() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Income,
        100_000_000_000_00, // $1 trillion
        Some("Lottery Win".to_string()),
        None,
        Utc::now(),
    );

    assert_eq!(transaction.amount, 100_000_000_000_00);
}

#[test]
fn test_negative_transaction_amount() {
    let account_id = Uuid::new_v4();
    let transaction = Transaction::new(
        account_id,
        TransactionType::Expense,
        -10000,
        None,
        None,
        Utc::now(),
    );

    assert_eq!(transaction.amount, -10000);
}
