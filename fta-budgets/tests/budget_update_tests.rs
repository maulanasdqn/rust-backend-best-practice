#![allow(clippy::inconsistent_digit_grouping, clippy::unreadable_literal)]

use chrono::Utc;
use fta_budgets::domain::{Budget, BudgetPeriod};
use uuid::Uuid;

#[test]
fn test_update_amount_changes_budget_amount() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Groceries".to_string(),
        50000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.update_amount(60000);

    assert_eq!(budget.amount, 60000);
}

#[test]
fn test_update_amount_can_decrease() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Groceries".to_string(),
        50000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.update_amount(30000);

    assert_eq!(budget.amount, 30000);
}

#[test]
fn test_update_amount_updates_timestamp() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    let original_updated_at = budget.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(10));
    budget.update_amount(15000);

    assert!(budget.updated_at > original_updated_at);
}

#[test]
fn test_multiple_amount_updates() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.update_amount(15000);
    budget.update_amount(20000);
    budget.update_amount(25000);

    assert_eq!(budget.amount, 25000);
}

#[test]
fn test_update_amount_to_max_value() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.update_amount(i64::MAX);
    assert_eq!(budget.amount, i64::MAX);
}

#[test]
fn test_update_amount_to_min_value() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.update_amount(i64::MIN);
    assert_eq!(budget.amount, i64::MIN);
}

#[test]
fn test_update_amount_to_negative() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.update_amount(-50000);
    assert_eq!(budget.amount, -50000);
}

#[test]
fn test_multiple_amount_updates_with_extreme_values() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.update_amount(i64::MAX);
    assert_eq!(budget.amount, i64::MAX);

    budget.update_amount(0);
    assert_eq!(budget.amount, 0);

    budget.update_amount(i64::MIN);
    assert_eq!(budget.amount, i64::MIN);

    budget.update_amount(50000);
    assert_eq!(budget.amount, 50000);
}
