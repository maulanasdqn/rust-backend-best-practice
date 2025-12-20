use axum::{extract::State, http::StatusCode, Json};
use fta_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{register_user, verify_email},
    infrastructure::http::dto::{
        MessageResponse, RegisterRequest, RegisterResponse, VerifyEmailRequest,
    },
};

use super::AuthAppState;

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Invalid request"),
        (status = 409, description = "User already exists")
    ),
    tag = "Authentication"
)]
pub async fn register_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), (StatusCode, Json<MessageResponse>)> {
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
            Json(RegisterResponse {
                user_id,
                message: "Registration successful. Please check your email for verification code."
                    .to_string(),
            }),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/verify-email",
    request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified successfully", body = MessageResponse),
        (status = 400, description = "Invalid or expired OTP code")
    ),
    tag = "Authentication"
)]
pub async fn verify_email_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<VerifyEmailRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = verify_email::VerifyEmail::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.email_verification_repository),
    );

    match use_case.execute(req.user_id, req.otp_code).await {
        Ok(()) => Ok(Json(MessageResponse::new("Email verified successfully"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}
