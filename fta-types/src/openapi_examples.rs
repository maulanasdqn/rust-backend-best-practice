/// `OpenAPI` response examples for Swagger documentation
///
/// This module provides standardized error response examples for use in utoipa/Swagger documentation.
/// Since utoipa's `example` attribute requires compile-time constants, we use macros to generate examples.
/// Macro to create a standardized error response example
///
/// # Example
/// ```
/// use fta_types::error_example;
///
/// let example = error_example!("User not found");
/// ```
#[macro_export]
macro_rules! error_example {
    ($message:expr) => {
        serde_json::json!({
            "message": $message,
            "stack_trace": [],
            "version": "v0.1.0"
        })
    };
}
