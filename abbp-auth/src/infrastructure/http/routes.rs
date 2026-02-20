use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use super::{
    handlers::{
        change_password_handler, disable_2fa_handler, enable_2fa_handler,
        google_oauth_callback_handler, google_oauth_login_handler, login_handler,
        logout_all_handler, logout_handler, refresh_token_handler, register_handler,
        request_password_reset_handler, reset_password_handler, verify_2fa_handler,
        verify_email_handler, AuthAppState,
    },
    middleware::{require_auth, AuthMiddlewareState},
};

pub fn auth_routes(state: Arc<AuthAppState>) -> Router {
    let auth_middleware_state = Arc::new(AuthMiddlewareState {
        jwt_service: state.jwt_service.clone(),
    });

    let public_routes = Router::new()
        .route("/register", post(register_handler))
        .route("/verify-email", post(verify_email_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_token_handler))
        .route("/logout", post(logout_handler))
        .route(
            "/password-reset/request",
            post(request_password_reset_handler),
        )
        .route("/password-reset/reset", post(reset_password_handler))
        .route("/google/login", get(google_oauth_login_handler))
        .route("/google/callback", post(google_oauth_callback_handler))
        .with_state(state.clone());

    let protected_routes = Router::new()
        .route("/logout-all", post(logout_all_handler))
        .route("/password/change", post(change_password_handler))
        .route("/2fa/enable", post(enable_2fa_handler))
        .route("/2fa/verify", post(verify_2fa_handler))
        .route("/2fa/disable", post(disable_2fa_handler))
        .layer(middleware::from_fn_with_state(
            auth_middleware_state,
            require_auth,
        ))
        .with_state(state);

    public_routes.merge(protected_routes)
}
