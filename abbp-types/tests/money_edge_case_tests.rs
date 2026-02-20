#![allow(clippy::float_cmp, clippy::inconsistent_digit_grouping)]

use abbp_types::Money;

#[test]
fn test_max_i64_value() {
    let money = Money::new(i64::MAX);
    assert_eq!(money.amount(), i64::MAX);
}

#[test]
fn test_min_i64_value() {
    let money = Money::new(i64::MIN);
    assert_eq!(money.amount(), i64::MIN);
}

#[test]
fn test_very_large_positive_amount() {
    let money = Money::new(999_999_999_999_99);
    assert!(money.is_positive());
    assert_eq!(money.to_decimal(), 999_999_999_999.99);
}

#[test]
fn test_very_large_negative_amount() {
    let money = Money::new(-999_999_999_999_99);
    assert!(money.is_negative());
    assert_eq!(money.to_decimal(), -999_999_999_999.99);
}

#[test]
fn test_boundary_near_overflow() {
    let money = Money::new(i64::MAX - 1000);
    let small = Money::new(500);
    let result = money.add(&small);
    assert_eq!(result.amount(), i64::MAX - 500);
}

#[test]
fn test_boundary_near_underflow() {
    let money = Money::new(i64::MIN + 1000);
    let small = Money::new(-500);
    let result = money.add(&small);
    assert_eq!(result.amount(), i64::MIN + 500);
}

#[test]
fn test_trillion_dollar_amount() {
    let money = Money::new(100_000_000_000_00);
    assert_eq!(money.to_decimal(), 100_000_000_000.0);
}
