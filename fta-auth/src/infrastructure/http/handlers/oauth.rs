use axum::{extract::State, http::StatusCode, Json};
use fta_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{google_oauth_callback, google_oauth_login},
    infrastructure::http::dto::{
        GoogleOAuthCallbackRequest, GoogleOAuthCallbackResponse, MessageResponse,
    },
};

use super::AuthAppState;

#[utoipa::path(
    get,
    path = "/api/v1/google/login",
    responses(
        (status = 200, description = "Authorization URL", body = MessageResponse)
    ),
    tag = "OAuth"
)]
pub async fn google_oauth_login_handler(
    State(state): State<Arc<AuthAppState>>,
) -> Json<MessageResponse> {
    let use_case = google_oauth_login::GoogleOAuthLogin::new(state.google_oauth_service.clone());

    let auth_url = use_case.execute();

    Json(MessageResponse::new(auth_url))
}

#[utoipa::path(
    post,
    path = "/api/v1/google/callback",
    request_body = GoogleOAuthCallbackRequest,
    responses(
        (status = 200, description = "OAuth login successful", body = GoogleOAuthCallbackResponse),
        (status = 400, description = "OAuth error")
    ),
    tag = "OAuth"
)]
pub async fn google_oauth_callback_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<GoogleOAuthCallbackRequest>,
) -> Result<Json<GoogleOAuthCallbackResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = google_oauth_callback::GoogleOAuthCallback::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.refresh_token_repository),
        state.google_oauth_service.clone(),
        state.jwt_service.clone(),
        state.password_hash_service.clone(),
    );

    match use_case.execute(req.code).await {
        Ok(response) => Ok(Json(GoogleOAuthCallbackResponse {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            is_new_user: response.is_new_user,
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}
