use axum::{extract::State, http::StatusCode, Json};
use abbp_types::responses::{ErrorResponse, MessageOnlyResponse};
use abbp_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{change_password, request_password_reset, reset_password},
    infrastructure::http::dto::{
        ChangePasswordRequest, RequestPasswordResetRequest, ResetPasswordRequest,
    },
};

use super::AuthAppState;

#[utoipa::path(
    post,
    path = "/api/v1/password-reset/request",
    request_body = RequestPasswordResetRequest,
    responses(
        (status = 200, description = "Reset email sent if user exists", body = MessageOnlyResponse)
    ),
    tag = "Authentication"
)]
pub async fn request_password_reset_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<RequestPasswordResetRequest>,
) -> Json<MessageOnlyResponse> {
    let use_case = request_password_reset::RequestPasswordReset::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.password_reset_token_repository),
        state.email_service.clone(),
        state.base_url.clone(),
    );

    let _ = use_case.execute(req.email).await;

    Json(MessageOnlyResponse::new(
        "If your email exists in our system, you will receive a password reset link",
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/password-reset/reset",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successfully", body = MessageOnlyResponse),
        (status = 400, description = "Invalid or expired token", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn reset_password_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<ResetPasswordRequest>,
) -> Result<Json<MessageOnlyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = reset_password::ResetPassword::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.password_reset_token_repository),
        state.password_hash_service.clone(),
    );

    match use_case.execute(req.token, req.new_password).await {
        Ok(()) => Ok(Json(MessageOnlyResponse::new("Password reset successfully"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/password/change",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully", body = MessageOnlyResponse),
        (status = 400, description = "Invalid current password", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn change_password_handler(
    State(state): State<Arc<AuthAppState>>,
    axum::extract::Extension(auth_user): axum::extract::Extension<
        crate::infrastructure::http::middleware::AuthUser,
    >,
    Validated(req): Validated<ChangePasswordRequest>,
) -> Result<Json<MessageOnlyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = change_password::ChangePassword::new(
        Arc::clone(&state.user_repository),
        state.password_hash_service.clone(),
    );

    match use_case
        .execute(auth_user.user_id, req.current_password, req.new_password)
        .await
    {
        Ok(()) => Ok(Json(MessageOnlyResponse::new("Password changed successfully"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}
