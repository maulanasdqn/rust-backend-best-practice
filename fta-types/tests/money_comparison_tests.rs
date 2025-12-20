use fta_types::Money;

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
fn test_comparing_very_close_amounts() {
    let a = Money::new(10000);
    let b = Money::new(10001);
    assert!(a < b);
    assert!(b > a);
    assert_ne!(a, b);
}
