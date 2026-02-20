use abbp_validation::{prelude::*, ObjectSchema, Validatable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterData {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyEmailRequest {
    pub user_id: Uuid,
    pub otp_code: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginData {
    pub token: TokenData,
    pub user: abbp_users::infrastructure::http::dto::UserResponse,
    pub requires_2fa: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenData {
    pub access_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RequestPasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Enable2FAData {
    pub secret: String,
    pub qr_code_svg: String,
    pub provisioning_uri: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Verify2FARequest {
    pub code: String,
    #[serde(default)]
    pub enable_on_success: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Disable2FARequest {
    pub password: String,
    pub code: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GoogleOAuthCallbackRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoogleOAuthData {
    pub access_token: String,
    pub refresh_token: String,
    pub is_new_user: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Validatable for RegisterRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("email", string().email().min(3).max(255))
            .field("password", string().min(8).max(128))
            .field("first_name", string().max(100).optional())
            .field("last_name", string().max(100).optional())
    }
}

impl Validatable for VerifyEmailRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("user_id", string())
            .field("otp_code", string().regex(r"^\d{6}$"))
    }
}

impl Validatable for LoginRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("email", string().email().min(3).max(255))
            .field("password", string().min(1))
    }
}

impl Validatable for RefreshTokenRequest {
    fn schema() -> ObjectSchema {
        object().field("refresh_token", string().min(1))
    }
}

impl Validatable for LogoutRequest {
    fn schema() -> ObjectSchema {
        object().field("refresh_token", string().min(1))
    }
}

impl Validatable for RequestPasswordResetRequest {
    fn schema() -> ObjectSchema {
        object().field("email", string().email().min(3).max(255))
    }
}

impl Validatable for ResetPasswordRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("token", string().min(1))
            .field("new_password", string().min(8).max(128))
    }
}

impl Validatable for ChangePasswordRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("current_password", string().min(1))
            .field("new_password", string().min(8).max(128))
    }
}

impl Validatable for Verify2FARequest {
    fn schema() -> ObjectSchema {
        object()
            .field("code", string().regex(r"^\d{6}$"))
            .field("enable_on_success", string().optional())
    }
}

impl Validatable for Disable2FARequest {
    fn schema() -> ObjectSchema {
        object()
            .field("password", string().min(1))
            .field("code", string().regex(r"^\d{6}$"))
    }
}

impl Validatable for GoogleOAuthCallbackRequest {
    fn schema() -> ObjectSchema {
        object().field("code", string().min(1))
    }
}
