use axum::{extract::State, http::StatusCode, Json};
use abbp_types::responses::{ErrorResponse, MessageOnlyResponse};
use abbp_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{logout, logout_all},
    infrastructure::http::dto::LogoutRequest,
};

use super::AuthAppState;

#[utoipa::path(
    post,
    path = "/api/v1/logout",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logout successful", body = MessageOnlyResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn logout_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<LogoutRequest>,
) -> Result<Json<MessageOnlyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = logout::Logout::new(Arc::clone(&state.refresh_token_repository));

    match use_case.execute(req.refresh_token).await {
        Ok(()) => Ok(Json(MessageOnlyResponse::new("Logout successful"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/logout-all",
    responses(
        (status = 200, description = "Logged out from all devices", body = MessageOnlyResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn logout_all_handler(
    State(state): State<Arc<AuthAppState>>,
    axum::extract::Extension(auth_user): axum::extract::Extension<
        crate::infrastructure::http::middleware::AuthUser,
    >,
) -> Result<Json<MessageOnlyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = logout_all::LogoutAll::new(Arc::clone(&state.refresh_token_repository));

    match use_case.execute(auth_user.user_id).await {
        Ok(()) => Ok(Json(MessageOnlyResponse::new(
            "Logged out from all devices successfully",
        ))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}
