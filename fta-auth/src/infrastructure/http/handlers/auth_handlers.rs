use axum::{extract::State, http::StatusCode, Json};
use fta_validation::Validated;
use std::sync::Arc;

use crate::{
    application::{
        change_password, disable_2fa, enable_2fa, google_oauth_callback, google_oauth_login, login,
        logout, logout_all, refresh_access_token, register_user, request_password_reset,
        reset_password, verify_2fa, verify_email,
    },
    domain::{EmailVerificationRepository, PasswordResetTokenRepository, RefreshTokenRepository},
    infrastructure::{
        http::dto::{
            ChangePasswordRequest, Disable2FARequest, Enable2FAResponse,
            GoogleOAuthCallbackRequest, GoogleOAuthCallbackResponse, LoginRequest, LoginResponse,
            LogoutRequest, MessageResponse, RefreshTokenRequest, RefreshTokenResponse,
            RegisterRequest, RegisterResponse, RequestPasswordResetRequest, ResetPasswordRequest,
            Verify2FARequest, VerifyEmailRequest,
        },
        services::{
            EmailService, GoogleOAuthService, JwtService, OtpService, PasswordHashService,
            TwoFactorService,
        },
    },
};

#[derive(Clone)]
pub struct AuthAppState {
    pub user_repository: Arc<dyn fta_users::domain::UserRepository>,
    pub refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    pub email_verification_repository: Arc<dyn EmailVerificationRepository>,
    pub password_reset_token_repository: Arc<dyn PasswordResetTokenRepository>,

    pub password_hash_service: PasswordHashService,
    pub jwt_service: JwtService,
    pub email_service: EmailService,
    pub google_oauth_service: GoogleOAuthService,
    pub two_factor_service: TwoFactorService,
    pub otp_service: OtpService,

    pub base_url: String,
}

impl std::fmt::Debug for AuthAppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthAppState")
            .field("user_repository", &"Arc<dyn UserRepository>")
            .field(
                "refresh_token_repository",
                &"Arc<dyn RefreshTokenRepository>",
            )
            .field(
                "email_verification_repository",
                &"Arc<dyn EmailVerificationRepository>",
            )
            .field(
                "password_reset_token_repository",
                &"Arc<dyn PasswordResetTokenRepository>",
            )
            .field("password_hash_service", &self.password_hash_service)
            .field("jwt_service", &self.jwt_service)
            .field("email_service", &self.email_service)
            .field("google_oauth_service", &self.google_oauth_service)
            .field("two_factor_service", &self.two_factor_service)
            .field("otp_service", &self.otp_service)
            .field("base_url", &self.base_url)
            .finish()
    }
}

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

#[utoipa::path(
    post,
    path = "/api/v1/logout",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logout successful", body = MessageResponse),
        (status = 400, description = "Invalid request")
    ),
    tag = "Authentication"
)]
pub async fn logout_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<LogoutRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = logout::Logout::new(Arc::clone(&state.refresh_token_repository));

    match use_case.execute(req.refresh_token).await {
        Ok(()) => Ok(Json(MessageResponse::new("Logout successful"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/logout-all",
    responses(
        (status = 200, description = "Logged out from all devices", body = MessageResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Authentication"
)]
pub async fn logout_all_handler(
    State(state): State<Arc<AuthAppState>>,
    axum::extract::Extension(auth_user): axum::extract::Extension<
        crate::infrastructure::http::middleware::AuthUser,
    >,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = logout_all::LogoutAll::new(Arc::clone(&state.refresh_token_repository));

    match use_case.execute(auth_user.user_id).await {
        Ok(()) => Ok(Json(MessageResponse::new(
            "Logged out from all devices successfully",
        ))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/password-reset/request",
    request_body = RequestPasswordResetRequest,
    responses(
        (status = 200, description = "Reset email sent if user exists", body = MessageResponse)
    ),
    tag = "Authentication"
)]
pub async fn request_password_reset_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<RequestPasswordResetRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = request_password_reset::RequestPasswordReset::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.password_reset_token_repository),
        state.email_service.clone(),
        state.base_url.clone(),
    );

    let _ = use_case.execute(req.email).await;

    Ok(Json(MessageResponse::new(
        "If your email exists in our system, you will receive a password reset link",
    )))
}

#[utoipa::path(
    post,
    path = "/api/v1/password-reset/reset",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successfully", body = MessageResponse),
        (status = 400, description = "Invalid or expired token")
    ),
    tag = "Authentication"
)]
pub async fn reset_password_handler(
    State(state): State<Arc<AuthAppState>>,
    Validated(req): Validated<ResetPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = reset_password::ResetPassword::new(
        Arc::clone(&state.user_repository),
        Arc::clone(&state.password_reset_token_repository),
        state.password_hash_service.clone(),
    );

    match use_case.execute(req.token, req.new_password).await {
        Ok(()) => Ok(Json(MessageResponse::new("Password reset successfully"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/password/change",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully", body = MessageResponse),
        (status = 400, description = "Invalid current password"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Authentication"
)]
pub async fn change_password_handler(
    State(state): State<Arc<AuthAppState>>,
    axum::extract::Extension(auth_user): axum::extract::Extension<
        crate::infrastructure::http::middleware::AuthUser,
    >,
    Validated(req): Validated<ChangePasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let use_case = change_password::ChangePassword::new(
        Arc::clone(&state.user_repository),
        state.password_hash_service.clone(),
    );

    match use_case
        .execute(auth_user.user_id, req.current_password, req.new_password)
        .await
    {
        Ok(()) => Ok(Json(MessageResponse::new("Password changed successfully"))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse::new(e.to_string())),
        )),
    }
}

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
        },
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
