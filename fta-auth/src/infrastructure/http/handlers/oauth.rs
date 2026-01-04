use axum::{extract::State, http::StatusCode, Json};
use fta_types::responses::{ErrorResponse, SingleResponse};
use fta_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{google_oauth_callback, google_oauth_login},
    infrastructure::http::dto::{GoogleOAuthCallbackRequest, GoogleOAuthData},
};

use super::AuthAppState;

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct GoogleOAuthLoginData {
    pub auth_url: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/google/login",
    responses(
        (status = 200, description = "Authorization URL", body = SingleResponse<GoogleOAuthLoginData>)
    ),
    tag = "OAuth"
)]
pub async fn google_oauth_login_handler(
    State(state): State<Arc<AuthAppState>>,
) -> Json<SingleResponse<GoogleOAuthLoginData>> {
    let use_case = google_oauth_login::GoogleOAuthLogin::new(state.google_oauth_service.clone());

    let auth_url = use_case.execute();

    Json(SingleResponse::with_message(
        "Google OAuth authorization URL",
        GoogleOAuthLoginData { auth_url },
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/google/callback",
    request_body = GoogleOAuthCallbackRequest,
    responses(
        (status = 200, description = "OAuth login successful", body = SingleResponse<GoogleOAuthData>),
        (status = 400, description = "OAuth error", body = ErrorResponse)
    ),
    tag = "OAuth"
)]
pub async fn google_oauth_callback_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<GoogleOAuthCallbackRequest>,
) -> Result<Json<SingleResponse<GoogleOAuthData>>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = google_oauth_callback::GoogleOAuthCallback::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.refresh_token_repository),
        state.google_oauth_service.clone(),
        state.jwt_service.clone(),
        state.password_hash_service.clone(),
    );

    match use_case.execute(req.code).await {
        Ok(response) => Ok(Json(SingleResponse::with_message(
            "OAuth login successful",
            GoogleOAuthData {
                access_token: response.access_token,
                refresh_token: response.refresh_token,
                is_new_user: response.is_new_user,
            },
        ))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(e.to_string())),
        )),
    }
}
