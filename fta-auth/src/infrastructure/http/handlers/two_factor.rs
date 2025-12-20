use axum::{extract::State, http::StatusCode, Json};
use fta_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{disable_2fa, enable_2fa, verify_2fa},
    infrastructure::http::dto::{
        Disable2FARequest, Enable2FAResponse, MessageResponse, Verify2FARequest,
    },
};

use super::AuthAppState;

#[utoipa::path(
    post,
    path = "/api/v1/2fa/enable",
    responses(
        (status = 200, description = "2FA setup initiated", body = Enable2FAResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Two-Factor Authentication"
)]
pub async fn enable_2fa_handler(
    State(state): State<Arc<AuthAppState>>,
    axum::extract::Extension(auth_user): axum::extract::Extension<
        crate::infrastructure::http::middleware::AuthUser,
    >,
) -> Result<Json<Enable2FAResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = enable_2fa::Enable2FA::new(
        Arc::clone(&state.user_repository),
        state.two_factor_service.clone(),
    );

    match use_case.execute(auth_user.user_id).await {
        Ok(response) => Ok(Json(Enable2FAResponse {
            secret: response.secret,
            qr_code_svg: response.qr_code_svg,
            provisioning_uri: response.provisioning_uri,
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/2fa/verify",
    request_body = Verify2FARequest,
    responses(
        (status = 200, description = "2FA code verified", body = MessageResponse),
        (status = 400, description = "Invalid 2FA code"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Two-Factor Authentication"
)]
pub async fn verify_2fa_handler(
    State(state): State<Arc<AuthAppState>>,
    axum::extract::Extension(auth_user): axum::extract::Extension<
        crate::infrastructure::http::middleware::AuthUser,
    >,
    Validated(req): Validated<Verify2FARequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = verify_2fa::Verify2FA::new(
        Arc::clone(&state.user_repository),
        state.two_factor_service.clone(),
    );

    match use_case
        .execute(auth_user.user_id, req.code, req.enable_on_success)
        .await
    {
        Ok(is_valid) => {
            if is_valid {
                Ok(Json(MessageResponse::new("2FA code verified successfully")))
            } else {
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(MessageResponse::new("Invalid 2FA code")),
                ))
            }
        }
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/2fa/disable",
    request_body = Disable2FARequest,
    responses(
        (status = 200, description = "2FA disabled successfully", body = MessageResponse),
        (status = 400, description = "Invalid password or 2FA code"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Two-Factor Authentication"
)]
pub async fn disable_2fa_handler(
    State(state): State<Arc<AuthAppState>>,
    axum::extract::Extension(auth_user): axum::extract::Extension<
        crate::infrastructure::http::middleware::AuthUser,
    >,
    Validated(req): Validated<Disable2FARequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = disable_2fa::Disable2FA::new(
        Arc::clone(&state.user_repository),
        state.two_factor_service.clone(),
        state.password_hash_service.clone(),
    );

    match use_case
        .execute(auth_user.user_id, req.password, req.code)
        .await
    {
        Ok(()) => Ok(Json(MessageResponse::new("2FA disabled successfully"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}
