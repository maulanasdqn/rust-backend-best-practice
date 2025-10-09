use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use super::{handlers::*, middleware::{require_auth, AuthMiddlewareState}};

/// Creates the authentication routes
///
/// # Routes
/// - POST /register - Register a new user
/// - POST /verify-email - Verify email with OTP
/// - POST /login - Login with email and password
/// - POST /refresh - Refresh access token
/// - POST /logout - Logout (invalidate single refresh token)
/// - POST /logout-all - Logout from all devices (requires auth)
/// - POST /password-reset/request - Request password reset email
/// - POST /password-reset/reset - Reset password with token
/// - POST /password/change - Change password (requires auth)
/// - POST /2fa/enable - Enable 2FA (requires auth)
/// - POST /2fa/verify - Verify 2FA code (requires auth)
/// - POST /2fa/disable - Disable 2FA (requires auth)
/// - GET /google/login - Get Google OAuth URL
/// - POST /google/callback - Handle Google OAuth callback
pub fn auth_routes(state: Arc<AuthAppState>) -> Router {
    // Create auth middleware state
    let auth_middleware_state = Arc::new(AuthMiddlewareState {
        jwt_service: state.jwt_service.clone(),
    });

    // Public routes
    let public_routes = Router::new()
        .route("/register", post(register_handler))
        .route("/verify-email", post(verify_email_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_token_handler))
        .route("/logout", post(logout_handler))
        .route("/password-reset/request", post(request_password_reset_handler))
        .route("/password-reset/reset", post(reset_password_handler))
        .route("/google/login", get(google_oauth_login_handler))
        .route("/google/callback", post(google_oauth_callback_handler))
        .with_state(state.clone());

    // Protected routes (require authentication)
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

    // Merge routes
    public_routes.merge(protected_routes)
}
