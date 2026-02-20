use chrono::{DateTime, Utc};
use abbp_budgets::domain::{Budget, BudgetPeriod};
use uuid::Uuid;

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
