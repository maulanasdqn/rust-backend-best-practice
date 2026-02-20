use chrono::Utc;
use abbp_budgets::domain::{Budget, BudgetPeriod};
use uuid::Uuid;

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
