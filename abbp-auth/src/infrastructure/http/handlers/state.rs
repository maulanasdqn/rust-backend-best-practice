use std::sync::Arc;

use crate::{
    domain::{EmailVerificationRepository, PasswordResetTokenRepository, RefreshTokenRepository},
    infrastructure::services::{
        EmailService, GoogleOAuthService, JwtService, OtpService, PasswordHashService,
        TwoFactorService,
    },
};

#[derive(Clone)]
pub struct AuthAppState {
    pub user_repository: Arc<dyn abbp_users::domain::UserRepository>,
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
