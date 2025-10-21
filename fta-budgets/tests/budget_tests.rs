#![allow(clippy::inconsistent_digit_grouping, clippy::unreadable_literal)]

use chrono::{DateTime, Utc};
use fta_budgets::domain::{Budget, BudgetPeriod};
use uuid::Uuid;

#[allow(dead_code)]
fn test_new_creates_budget_with_correct_fields() {
    let user_id = Uuid::new_v4();
    let start_date = Utc::now();
    let budget = Budget::new(
        user_id,
        "Groceries".to_string(),
        50000,
        BudgetPeriod::Monthly,
        start_date,
    );

    assert_eq!(budget.user_id, user_id);
    assert_eq!(budget.category, "Groceries");
    assert_eq!(budget.amount, 50000);
    assert_eq!(budget.period, BudgetPeriod::Monthly);
    assert_eq!(budget.start_date, start_date);
    assert!(budget.is_active);
    assert!(budget.end_date.is_none());
}

#[test]
fn test_new_budget_is_active_by_default() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Entertainment".to_string(),
        10000,
        BudgetPeriod::Weekly,
        Utc::now(),
    );

    assert!(budget.is_active);
}

#[test]
fn test_new_budget_has_no_end_date() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Transport".to_string(),
        20000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert!(budget.end_date.is_none());
}

#[test]
fn test_new_budget_timestamps_are_set() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Daily,
        Utc::now(),
    );

    assert_eq!(budget.created_at, budget.updated_at);
}

#[test]
fn test_budget_period_daily() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Test".to_string(),
        1000,
        BudgetPeriod::Daily,
        Utc::now(),
    );

    assert_eq!(budget.period, BudgetPeriod::Daily);
}

#[test]
fn test_budget_period_weekly() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Test".to_string(),
        5000,
        BudgetPeriod::Weekly,
        Utc::now(),
    );

    assert_eq!(budget.period, BudgetPeriod::Weekly);
}

#[test]
fn test_budget_period_monthly() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Test".to_string(),
        20000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert_eq!(budget.period, BudgetPeriod::Monthly);
}

#[test]
fn test_budget_period_yearly() {
    let user_id = Uuid::new_v4();
    let budget = Budget::new(
        user_id,
        "Test".to_string(),
        100000,
        BudgetPeriod::Yearly,
        Utc::now(),
    );

    assert_eq!(budget.period, BudgetPeriod::Yearly);
}

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
fn test_deactivate_sets_is_active_to_false() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    let end_date = Utc::now();
    budget.deactivate(end_date);

    assert!(!budget.is_active);
}

#[test]
fn test_deactivate_sets_end_date() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    let end_date = Utc::now();
    budget.deactivate(end_date);

    assert_eq!(budget.end_date, Some(end_date));
}

#[test]
fn test_deactivate_updates_timestamp() {
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
    budget.deactivate(Utc::now());

    assert!(budget.updated_at > original_updated_at);
}

#[test]
fn test_activate_sets_is_active_to_true() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.deactivate(Utc::now());
    assert!(!budget.is_active);

    budget.activate();
    assert!(budget.is_active);
}

#[test]
fn test_activate_clears_end_date() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.deactivate(Utc::now());
    assert!(budget.end_date.is_some());

    budget.activate();
    assert!(budget.end_date.is_none());
}

#[test]
fn test_activate_updates_timestamp() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    budget.deactivate(Utc::now());
    let deactivated_updated_at = budget.updated_at;
    std::thread::sleep(std::time::Duration::from_millis(10));
    budget.activate();

    assert!(budget.updated_at > deactivated_updated_at);
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
fn test_deactivate_and_reactivate_cycle() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    assert!(budget.is_active);
    assert!(budget.end_date.is_none());

    budget.deactivate(Utc::now());
    assert!(!budget.is_active);
    assert!(budget.end_date.is_some());

    budget.activate();
    assert!(budget.is_active);
    assert!(budget.end_date.is_none());
}

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
fn test_deactivate_with_very_old_end_date() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    let old_date = DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    budget.deactivate(old_date);
    assert_eq!(budget.end_date, Some(old_date));
}

#[test]
fn test_deactivate_with_far_future_end_date() {
    let user_id = Uuid::new_v4();
    let mut budget = Budget::new(
        user_id,
        "Test".to_string(),
        10000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );

    let future_date = DateTime::parse_from_rfc3339("2099-12-31T23:59:59Z")
        .unwrap()
        .with_timezone(&Utc);

    budget.deactivate(future_date);
    assert_eq!(budget.end_date, Some(future_date));
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

#[test]
fn test_budget_period_edge_cases_all_types() {
    let user_id = Uuid::new_v4();

    let daily = Budget::new(
        user_id,
        "Daily".to_string(),
        1000,
        BudgetPeriod::Daily,
        Utc::now(),
    );
    assert_eq!(daily.period, BudgetPeriod::Daily);

    let weekly = Budget::new(
        user_id,
        "Weekly".to_string(),
        5000,
        BudgetPeriod::Weekly,
        Utc::now(),
    );
    assert_eq!(weekly.period, BudgetPeriod::Weekly);

    let monthly = Budget::new(
        user_id,
        "Monthly".to_string(),
        20000,
        BudgetPeriod::Monthly,
        Utc::now(),
    );
    assert_eq!(monthly.period, BudgetPeriod::Monthly);

    let yearly = Budget::new(
        user_id,
        "Yearly".to_string(),
        240000,
        BudgetPeriod::Yearly,
        Utc::now(),
    );
    assert_eq!(yearly.period, BudgetPeriod::Yearly);
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
