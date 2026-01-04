use axum::{extract::State, http::StatusCode, Json};
use fta_types::responses::{ErrorResponse, MessageOnlyResponse, SingleResponse};
use fta_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{register_user, verify_email},
    infrastructure::http::dto::{RegisterData, RegisterRequest, VerifyEmailRequest},
};

use super::AuthAppState;

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = SingleResponse<RegisterData>),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn register_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<RegisterRequest>,
) -> Result<(StatusCode, Json<SingleResponse<RegisterData>>), (StatusCode, Json<ErrorResponse>)> {
    let use_case = register_user::RegisterUser::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.email_verification_repository),
        state.password_hash_service.clone(),
        state.otp_service.clone(),
        state.email_service.clone(),
    );

    match use_case
        .execute(req.email, req.password, req.first_name, req.last_name)
        .await
    {
        Ok(user_id) => Ok((
            StatusCode::CREATED,
            Json(SingleResponse::with_message(
                "Registration successful. Please check your email for verification code.",
                RegisterData { user_id },
            )),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/verify-email",
    request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified successfully", body = MessageOnlyResponse),
        (status = 400, description = "Invalid or expired OTP code", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn verify_email_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<VerifyEmailRequest>,
) -> Result<Json<MessageOnlyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = verify_email::VerifyEmail::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.email_verification_repository),
    );

    match use_case.execute(req.user_id, req.otp_code).await {
        Ok(()) => Ok(Json(MessageOnlyResponse::new("Email verified successfully"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}
