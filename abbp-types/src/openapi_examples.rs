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
