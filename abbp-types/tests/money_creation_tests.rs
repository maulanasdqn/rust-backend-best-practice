#![allow(clippy::float_cmp)]

use abbp_types::Money;

#[test]
fn test_new_creates_money_with_correct_amount() {
    let money = Money::new(1000);
    assert_eq!(money.amount(), 1000);
}

#[test]
fn test_new_with_zero() {
    let money = Money::new(0);
    assert_eq!(money.amount(), 0);
}

#[test]
fn test_new_with_negative_amount() {
    let money = Money::new(-500);
    assert_eq!(money.amount(), -500);
}

#[test]
fn test_from_decimal_converts_correctly() {
    let money = Money::from_decimal(10.50);
    assert_eq!(money.amount(), 1050);
}

#[test]
fn test_from_decimal_with_zero() {
    let money = Money::from_decimal(0.0);
    assert_eq!(money.amount(), 0);
}

#[test]
fn test_from_decimal_with_negative() {
    let money = Money::from_decimal(-25.99);
    assert_eq!(money.amount(), -2599);
}

#[test]
fn test_from_decimal_rounds_correctly() {
    let money = Money::from_decimal(10.556);
    assert_eq!(money.amount(), 1056);
}

#[test]
fn test_from_decimal_rounding_up() {
    let money = Money::from_decimal(10.995);
    assert_eq!(money.amount(), 1100);
}

#[test]
fn test_from_decimal_rounding_down() {
    let money = Money::from_decimal(10.994);
    assert_eq!(money.amount(), 1099);
}

#[test]
fn test_from_decimal_with_very_small_fraction() {
    let money = Money::from_decimal(0.001);
    assert_eq!(money.amount(), 0);
}

#[test]
fn test_from_decimal_with_negative_very_small_fraction() {
    let money = Money::from_decimal(-0.001);
    assert_eq!(money.amount(), 0);
}

#[test]
fn test_from_decimal_negative_zero() {
    let money = Money::from_decimal(-0.0);
    assert!(money.is_zero());
    assert_eq!(money.amount(), 0);
}

#[test]
fn test_from_decimal_with_many_decimal_places() {
    let money = Money::from_decimal(123.456789);
    assert_eq!(money.amount(), 12346);
}
