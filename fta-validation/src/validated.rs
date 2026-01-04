use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use fta_types::responses::ErrorResponse;
use serde::de::DeserializeOwned;
use serde_json::Value;
use zod_rs::{ObjectSchema, Schema};

pub trait Validatable: Sized + DeserializeOwned {
    fn schema() -> ObjectSchema;
}

#[derive(Debug)]
pub struct Validated<T>(pub T);

impl<T, S> FromRequest<S> for Validated<T>
where
    T: Validatable,
    S: Send + Sync,
{
    type Rejection = ValidationErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value): Json<Value> = Json::from_request(req, state)
            .await
            .map_err(|_| ValidationErrorResponse::new("Invalid JSON body"))?;

        let schema = T::schema();
        match schema.safe_parse(&value) {
            Ok(_validated_value) => {
                let data: T = serde_json::from_value(value).map_err(|e| {
                    ValidationErrorResponse::new(&format!("Deserialization error: {e}"))
                })?;
                Ok(Self(data))
            }
            Err(errors) => {
                let error_msg = format!("Validation failed: {errors}");
                Err(ValidationErrorResponse::new(&error_msg))
            }
        }
    }
}

#[derive(Debug)]
pub struct ValidationErrorResponse {
    inner: ErrorResponse,
}

impl ValidationErrorResponse {
    fn new(message: &str) -> Self {
        Self {
            inner: ErrorResponse::new(message),
        }
    }
}

impl IntoResponse for ValidationErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self.inner)).into_response()
    }
}
