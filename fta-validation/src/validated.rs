use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use zod_rs::{ObjectSchema, Schema};

/// Trait for types that can be validated using zod-rs schemas
pub trait Validatable: Sized + DeserializeOwned {
    /// Returns the zod-rs schema for this type
    fn schema() -> ObjectSchema;
}

/// Axum extractor that validates the request body using zod-rs
///
/// # Example
/// ```rust,ignore
/// async fn create_user(
///   Validated(req): Validated<CreateUserRequest>
/// ) -> Result<Json<UserResponse>, AppError> {
///   // req is already validated!
/// }
/// ```
#[derive(Debug)]
pub struct Validated<T>(pub T);

impl<T, S> FromRequest<S> for Validated<T>
where
    T: Validatable,
    S: Send + Sync,
{
    type Rejection = ValidationErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract JSON body
        let Json(value): Json<Value> = Json::from_request(req, state)
            .await
            .map_err(|_| ValidationErrorResponse::new("Invalid JSON body"))?;

        // Get schema and validate
        let schema = T::schema();
        match schema.safe_parse(&value) {
            Ok(_validated_value) => {
                // Deserialize into target type
                let data: T = serde_json::from_value(value).map_err(|e| {
                    ValidationErrorResponse::new(&format!("Deserialization error: {e}"))
                })?;
                Ok(Self(data))
            },
            Err(errors) => {
                // Format validation errors
                let error_msg = format!("Validation failed: {errors}");
                Err(ValidationErrorResponse::new(&error_msg))
            },
        }
    }
}

/// Response type for validation errors
#[derive(Debug)]
pub struct ValidationErrorResponse {
    message: String,
}

impl ValidationErrorResponse {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ValidationErrorResponse {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
          "error": self.message
        });
        (StatusCode::BAD_REQUEST, Json(body)).into_response()
    }
}
