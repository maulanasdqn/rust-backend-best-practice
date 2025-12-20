#![allow(clippy::float_cmp)]

use fta_types::Money;

#[test]
fn test_to_decimal_converts_correctly() {
    let money = Money::new(1050);
    assert_eq!(money.to_decimal(), 10.50);
}

#[test]
fn test_to_decimal_with_zero() {
    let money = Money::new(0);
    assert_eq!(money.to_decimal(), 0.0);
}

#[test]
fn test_to_decimal_with_negative() {
    let money = Money::new(-2599);
    assert_eq!(money.to_decimal(), -25.99);
}

#[test]
fn test_round_trip_conversion() {
    let original = 123.45;
    let money = Money::from_decimal(original);
    let result = money.to_decimal();
    assert_eq!(result, original);
}

#[test]
fn test_round_trip_conversion_negative() {
    let original = -67.89;
    let money = Money::from_decimal(original);
    let result = money.to_decimal();
    assert_eq!(result, original);
}

#[test]
fn test_large_amount() {
    let money = Money::new(1_000_000_00);
    assert_eq!(money.to_decimal(), 1_000_000.0);
}

#[test]
fn test_small_cents() {
    let money = Money::new(1);
    assert_eq!(money.to_decimal(), 0.01);
}

#[test]
fn test_money_with_exactly_one_cent() {
    let money = Money::new(1);
    assert!(money.is_positive());
    assert_eq!(money.to_decimal(), 0.01);
}

#[test]
fn test_money_with_exactly_negative_one_cent() {
    let money = Money::new(-1);
    assert!(money.is_negative());
    assert_eq!(money.to_decimal(), -0.01);
}

#[test]
fn test_round_trip_with_fractional_cents() {
    let original = 99.99;
    let money = Money::from_decimal(original);
    let result = money.to_decimal();
    assert_eq!(result, original);
}
