#![allow(clippy::inconsistent_digit_grouping)]

use chrono::{DateTime, Utc};
use abbp_transactions::domain::{Transaction, TransactionType};
use uuid::Uuid;

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
        100_000_000_000_00,
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
