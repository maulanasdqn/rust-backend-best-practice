use axum::{extract::State, http::StatusCode, Json};
use fta_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{login, refresh_access_token},
    infrastructure::http::dto::{
        LoginRequest, LoginResponse, MessageResponse, RefreshTokenRequest, RefreshTokenResponse,
    },
};

use super::AuthAppState;

#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Authentication"
)]
pub async fn login_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = login::Login::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.refresh_token_repository),
        state.password_hash_service.clone(),
        state.jwt_service.clone(),
    );

    match use_case.execute(req.email, req.password).await {
        Ok(response) => Ok(Json(LoginResponse {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            requires_2fa: response.requires_2fa,
        })),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshTokenResponse),
        (status = 401, description = "Invalid or expired refresh token")
    ),
    tag = "Authentication"
)]
pub async fn refresh_token_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = refresh_access_token::RefreshAccessToken::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.refresh_token_repository),
        state.jwt_service.clone(),
    );

    match use_case.execute(req.refresh_token).await {
        Ok(access_token) => Ok(Json(RefreshTokenResponse { access_token })),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}
