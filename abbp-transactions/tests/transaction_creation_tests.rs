use chrono::Utc;
use abbp_transactions::domain::{Transaction, TransactionType};
use uuid::Uuid;

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
