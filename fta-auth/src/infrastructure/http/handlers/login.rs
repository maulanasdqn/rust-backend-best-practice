use axum::{extract::State, http::StatusCode, Json};
use fta_types::responses::{ErrorResponse, SingleResponse};
use fta_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{login, refresh_access_token},
    infrastructure::http::dto::{
        LoginData, LoginRequest, RefreshTokenData, RefreshTokenRequest, TokenData,
    },
};

use super::AuthAppState;

#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = SingleResponse<LoginData>),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn login_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<LoginRequest>,
) -> Result<Json<SingleResponse<LoginData>>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = login::Login::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.refresh_token_repository),
        state.password_hash_service.clone(),
        state.jwt_service.clone(),
    );

    match use_case.execute(req.email, req.password).await {
        Ok(result) => Ok(Json(SingleResponse::with_message(
            "Login successful",
            LoginData {
                token: TokenData {
                    access_token: result.access_token,
                    refresh_token: result.refresh_token,
                },
                user: result.user.into(),
                requires_2fa: result.requires_2fa,
            },
        ))),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = SingleResponse<RefreshTokenData>),
        (status = 401, description = "Invalid or expired refresh token", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn refresh_token_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<RefreshTokenRequest>,
) -> Result<Json<SingleResponse<RefreshTokenData>>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = refresh_access_token::RefreshAccessToken::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.refresh_token_repository),
        state.jwt_service.clone(),
    );

    match use_case.execute(req.refresh_token).await {
        Ok(access_token) => Ok(Json(SingleResponse::with_message(
            "Token refreshed successfully",
            RefreshTokenData { access_token },
        ))),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}
