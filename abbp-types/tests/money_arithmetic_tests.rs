#![allow(clippy::float_cmp)]

use abbp_types::Money;

#[test]
fn test_add_positive_amounts() {
    let a = Money::new(1000);
    let b = Money::new(500);
    let result = a.add(&b);
    assert_eq!(result.amount(), 1500);
}

#[test]
fn test_add_with_zero() {
    let a = Money::new(1000);
    let b = Money::new(0);
    let result = a.add(&b);
    assert_eq!(result.amount(), 1000);
}

#[test]
fn test_add_negative_amounts() {
    let a = Money::new(-500);
    let b = Money::new(-300);
    let result = a.add(&b);
    assert_eq!(result.amount(), -800);
}

#[test]
fn test_add_mixed_signs() {
    let a = Money::new(1000);
    let b = Money::new(-400);
    let result = a.add(&b);
    assert_eq!(result.amount(), 600);
}

#[test]
fn test_subtract_positive_amounts() {
    let a = Money::new(1000);
    let b = Money::new(400);
    let result = a.subtract(&b);
    assert_eq!(result.amount(), 600);
}

#[test]
fn test_subtract_resulting_in_negative() {
    let a = Money::new(500);
    let b = Money::new(800);
    let result = a.subtract(&b);
    assert_eq!(result.amount(), -300);
}

#[test]
fn test_subtract_with_zero() {
    let a = Money::new(1000);
    let b = Money::new(0);
    let result = a.subtract(&b);
    assert_eq!(result.amount(), 1000);
}

#[test]
fn test_add_preserves_precision() {
    let a = Money::from_decimal(10.99);
    let b = Money::from_decimal(5.01);
    let result = a.add(&b);
    assert_eq!(result.to_decimal(), 16.00);
}

#[test]
fn test_subtract_preserves_precision() {
    let a = Money::from_decimal(20.50);
    let b = Money::from_decimal(10.25);
    let result = a.subtract(&b);
    assert_eq!(result.to_decimal(), 10.25);
}

#[test]
fn test_add_resulting_in_zero() {
    let a = Money::new(1000);
    let b = Money::new(-1000);
    let result = a.add(&b);
    assert!(result.is_zero());
}

#[test]
fn test_subtract_resulting_in_zero() {
    let a = Money::new(1000);
    let b = Money::new(1000);
    let result = a.subtract(&b);
    assert!(result.is_zero());
}

#[test]
fn test_multiple_additions_precision() {
    let mut result = Money::new(0);

    for _ in 0..100 {
        result = result.add(&Money::new(1));
    }
    assert_eq!(result.amount(), 100);
    assert_eq!(result.to_decimal(), 1.00);
}

#[test]
fn test_alternating_add_subtract() {
    let start = Money::new(10000);
    let amount = Money::new(500);

    let result = start.add(&amount).subtract(&amount);
    assert_eq!(result.amount(), 10000);
}

#[test]
fn test_typical_financial_operations() {
    let initial_balance = Money::from_decimal(1000.00);
    let deposit = Money::from_decimal(250.50);
    let withdrawal = Money::from_decimal(75.25);

    let after_deposit = initial_balance.add(&deposit);
    let final_balance = after_deposit.subtract(&withdrawal);

    assert_eq!(final_balance.to_decimal(), 1175.25);
    assert!(final_balance.is_positive());
}
