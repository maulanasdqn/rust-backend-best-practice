#![allow(clippy::inconsistent_digit_grouping, clippy::unreadable_literal)]

use chrono::{DateTime, Utc};
use abbp_budgets::domain::{Budget, BudgetPeriod};
use uuid::Uuid;

#[test]
fn test_budget_with_zero_amount() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Test".to_string(),
        0,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.amount, 0);
}

#[test]
fn test_start_date_is_preserved() {
    let user_id = Uuid::new_v4();
    let specific_date = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        specific_date,
    );

    assert_eq!(budget.start_date, specific_date);
}

#[test]
fn test_budget_with_max_i64_amount() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Max Budget".to_string(),
        i64::MAX,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.amount, i64::MAX);
}

#[test]
fn test_budget_with_min_i64_amount() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Min Budget".to_string(),
        i64::MIN,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.amount, i64::MIN);
}

#[test]
fn test_budget_with_negative_amount() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Negative Budget".to_string(),
        -50000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.amount, -50000);
}

#[test]
fn test_budget_with_empty_category() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        String::new(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.category, "");
}

#[test]
fn test_budget_with_very_long_category() {
    let user_id = Uuid::new_v4();
    let long_category = "C".repeat(10000);
    let budget = Budget::new(
        user_id,
        long_category.clone(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.category, long_category);
}

#[test]
fn test_budget_with_unicode_category() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "食品预算 🍕 💰".to_string(),
        50000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.category, "食品预算 🍕 💰");
}

#[test]
fn test_budget_with_special_characters_in_category() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Category!@#$%^&*()".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.category, "Category!@#$%^&*()");
}

#[test]
fn test_budget_with_very_old_start_date() {
    let user_id = Uuid::new_v4();
    let old_date = DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let budget = Budget::new(
        user_id,
        "Old Budget".to_string(),
        10000,
        BudgetPeriod::Monthly,
        old_date,
    );

    assert_eq!(budget.start_date, old_date);
}

#[test]
fn test_budget_with_far_future_start_date() {
    let user_id = Uuid::new_v4();
    let future_date = DateTime::parse_from_rfc3339("2099-12-31T23:59:59Z")
        .unwrap()
        .with_timezone(&Utc);

    let budget = Budget::new(
        user_id,
        "Future Budget".to_string(),
        10000,
        BudgetPeriod::Yearly,
        future_date,
    );

    assert_eq!(budget.start_date, future_date);
}

#[test]
fn test_very_large_budget_amount() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Corporate Budget".to_string(),
        100_000_000_000_00,
        BudgetPeriod::Yearly,
        Utc::now(),
    );

    assert_eq!(budget.amount, 100_000_000_000_00);
}
