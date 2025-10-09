use fta_validation::prelude::*;
use serde_json::json;

#[test]
fn test_string_validation_min_max() {
    let schema = string().min(3).max(10);

    // Valid cases
    assert!(schema.safe_parse(&json!("hello")).is_ok());
    assert!(schema.safe_parse(&json!("abc")).is_ok());

    // Invalid cases
    assert!(schema.safe_parse(&json!("ab")).is_err()); // too short
    assert!(schema.safe_parse(&json!("this is too long")).is_err()); // too long
}

#[test]
fn test_email_validation() {
    let schema = string().email();

    // Valid cases
    assert!(schema.safe_parse(&json!("user@example.com")).is_ok());
    assert!(schema.safe_parse(&json!("test.user@domain.co.uk")).is_ok());

    // Invalid cases
    assert!(schema.safe_parse(&json!("notanemail")).is_err());
    assert!(schema.safe_parse(&json!("@example.com")).is_err());
    assert!(schema.safe_parse(&json!("user@")).is_err());
}

#[test]
fn test_number_validation() {
    let schema = number().min(0.0).max(100.0);

    // Valid cases
    assert!(schema.safe_parse(&json!(50)).is_ok());
    assert!(schema.safe_parse(&json!(0)).is_ok());
    assert!(schema.safe_parse(&json!(100)).is_ok());

    // Invalid cases
    assert!(schema.safe_parse(&json!(-1)).is_err());
    assert!(schema.safe_parse(&json!(101)).is_err());
}

#[test]
fn test_object_validation() {
    let schema = object()
        .field("name", string().min(2).max(50))
        .field("age", number().min(0.0).max(120.0).int())
        .field("email", string().email());

    // Valid case
    let valid_data = json!({
      "name": "John Doe",
      "age": 30,
      "email": "john@example.com"
    });
    assert!(schema.safe_parse(&valid_data).is_ok());

    // Invalid cases
    let invalid_name = json!({
      "name": "J",
      "age": 30,
      "email": "john@example.com"
    });
    assert!(schema.safe_parse(&invalid_name).is_err());

    let invalid_email = json!({
      "name": "John Doe",
      "age": 30,
      "email": "notanemail"
    });
    assert!(schema.safe_parse(&invalid_email).is_err());
}

#[test]
fn test_optional_fields() {
    let schema = object()
        .field("required", string())
        .field("optional", string().optional());

    // Both fields present
    assert!(schema
        .safe_parse(&json!({
          "required": "value",
          "optional": "value"
        }))
        .is_ok());

    // Only required field with null optional
    assert!(schema
        .safe_parse(&json!({
          "required": "value",
          "optional": null
        }))
        .is_ok());

    // Missing required field
    assert!(schema
        .safe_parse(&json!({
          "optional": "value"
        }))
        .is_err());
}

#[test]
fn test_positive_number() {
    let schema = number().positive();

    // Valid cases
    assert!(schema.safe_parse(&json!(1)).is_ok());
    assert!(schema.safe_parse(&json!(100.5)).is_ok());

    // Invalid cases
    assert!(schema.safe_parse(&json!(0)).is_err());
    assert!(schema.safe_parse(&json!(-1)).is_err());
}
