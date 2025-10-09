#![allow(
    clippy::float_cmp,
    clippy::inconsistent_digit_grouping,
    clippy::unreadable_literal
)]

use fta_types::Money;

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
    // Test rounding behavior
    let money = Money::from_decimal(10.556); // Should round to 10.56
    assert_eq!(money.amount(), 1056);
}

// Arithmetic operation tests
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

// Conversion tests
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

// State checking tests
#[test]
fn test_is_positive_with_positive_amount() {
    let money = Money::new(100);
    assert!(money.is_positive());
}

#[test]
fn test_is_positive_with_zero() {
    let money = Money::new(0);
    assert!(!money.is_positive());
}

#[test]
fn test_is_positive_with_negative_amount() {
    let money = Money::new(-100);
    assert!(!money.is_positive());
}

#[test]
fn test_is_negative_with_negative_amount() {
    let money = Money::new(-100);
    assert!(money.is_negative());
}

#[test]
fn test_is_negative_with_zero() {
    let money = Money::new(0);
    assert!(!money.is_negative());
}

#[test]
fn test_is_negative_with_positive_amount() {
    let money = Money::new(100);
    assert!(!money.is_negative());
}

#[test]
fn test_is_zero_with_zero() {
    let money = Money::new(0);
    assert!(money.is_zero());
}

#[test]
fn test_is_zero_with_positive_amount() {
    let money = Money::new(100);
    assert!(!money.is_zero());
}

#[test]
fn test_is_zero_with_negative_amount() {
    let money = Money::new(-100);
    assert!(!money.is_zero());
}

// Precision and edge case tests
#[test]
fn test_large_amount() {
    let money = Money::new(1_000_000_00); // $1,000,000.00
    assert_eq!(money.to_decimal(), 1_000_000.0);
}

#[test]
fn test_small_cents() {
    let money = Money::new(1); // $0.01
    assert_eq!(money.to_decimal(), 0.01);
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
fn test_money_equality() {
    let a = Money::new(1000);
    let b = Money::new(1000);
    assert_eq!(a, b);
}

#[test]
fn test_money_inequality() {
    let a = Money::new(1000);
    let b = Money::new(1001);
    assert_ne!(a, b);
}

#[test]
fn test_money_ordering() {
    let a = Money::new(500);
    let b = Money::new(1000);
    assert!(a < b);
    assert!(b > a);
}

#[test]
fn test_typical_financial_operations() {
    // Simulate a typical transaction scenario
    let initial_balance = Money::from_decimal(1000.00);
    let deposit = Money::from_decimal(250.50);
    let withdrawal = Money::from_decimal(75.25);

    let after_deposit = initial_balance.add(&deposit);
    let final_balance = after_deposit.subtract(&withdrawal);

    assert_eq!(final_balance.to_decimal(), 1175.25);
    assert!(final_balance.is_positive());
}

// Edge case tests
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
    let money = Money::new(999_999_999_999_99); // $9,999,999,999,999.99
    assert!(money.is_positive());
    assert_eq!(money.to_decimal(), 999_999_999_999.99);
}

#[test]
fn test_very_large_negative_amount() {
    let money = Money::new(-999_999_999_999_99); // -$9,999,999,999,999.99
    assert!(money.is_negative());
    assert_eq!(money.to_decimal(), -999_999_999_999.99);
}

#[test]
fn test_from_decimal_with_very_small_fraction() {
    let money = Money::from_decimal(0.001); // Should round to 0.00
    assert_eq!(money.amount(), 0);
}

#[test]
fn test_from_decimal_with_negative_very_small_fraction() {
    let money = Money::from_decimal(-0.001); // Should round to 0.00
    assert_eq!(money.amount(), 0);
}

#[test]
fn test_from_decimal_rounding_up() {
    let money = Money::from_decimal(10.995); // Should round to 10.10
    assert_eq!(money.amount(), 1100);
}

#[test]
fn test_from_decimal_rounding_down() {
    let money = Money::from_decimal(10.994); // Should round to 10.99
    assert_eq!(money.amount(), 1099);
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
    // Add 0.01 one hundred times
    for _ in 0..100 {
        result = result.add(&Money::new(1));
    }
    assert_eq!(result.amount(), 100); // Should be exactly $1.00
    assert_eq!(result.to_decimal(), 1.00);
}

#[test]
fn test_alternating_add_subtract() {
    let start = Money::new(10000);
    let amount = Money::new(500);

    let result = start.add(&amount).subtract(&amount);
    assert_eq!(result.amount(), 10000); // Should return to original
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
fn test_from_decimal_with_many_decimal_places() {
    let money = Money::from_decimal(123.456789);
    // Should round to 123.46
    assert_eq!(money.amount(), 12346);
}

#[test]
fn test_boundary_near_overflow() {
    // Test near i64::MAX without overflow
    let money = Money::new(i64::MAX - 1000);
    let small = Money::new(500);
    let result = money.add(&small);
    assert_eq!(result.amount(), i64::MAX - 500);
}

#[test]
fn test_boundary_near_underflow() {
    // Test near i64::MIN without underflow
    let money = Money::new(i64::MIN + 1000);
    let small = Money::new(-500);
    let result = money.add(&small);
    assert_eq!(result.amount(), i64::MIN + 500);
}

#[test]
fn test_comparing_very_close_amounts() {
    let a = Money::new(10000);
    let b = Money::new(10001);
    assert!(a < b);
    assert!(b > a);
    assert_ne!(a, b);
}

#[test]
fn test_from_decimal_negative_zero() {
    let money = Money::from_decimal(-0.0);
    assert!(money.is_zero());
    assert_eq!(money.amount(), 0);
}

#[test]
fn test_round_trip_with_fractional_cents() {
    // Test that values with fractional cents round-trip correctly
    let original = 99.99;
    let money = Money::from_decimal(original);
    let result = money.to_decimal();
    assert_eq!(result, original);
}

#[test]
fn test_trillion_dollar_amount() {
    let money = Money::new(100_000_000_000_00); // $1 trillion (100 trillion cents)
    assert_eq!(money.to_decimal(), 100_000_000_000.0);
}
