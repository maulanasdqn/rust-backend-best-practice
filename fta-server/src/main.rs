mod api_doc;
mod config;
mod health;
mod logger;
mod repositories;
mod router;
mod use_cases;

use std::sync::Arc;

use anyhow::Result;

use fta_auth::{
    AuthAppState, EmailService, GoogleOAuthService, JwtService, OtpService, PasswordHashService,
    PostgresEmailVerificationRepository, PostgresPasswordResetTokenRepository,
    PostgresRefreshTokenRepository, TwoFactorService,
};
use fta_database::create_pool;
use fta_users::PostgresUserRepository;

use config::{load_environment_config, Config};
use logger::init_logger;
use repositories::setup_repositories;
use router::build_router;
use use_cases::init_use_cases;

async fn setup_database_pool(database_url: &str) -> Result<fta_database::DbPool> {
    let db_pool = create_pool(database_url).await?;
    tracing::info!("Database connection established");
    Ok(db_pool)
}

fn setup_auth_state(
    config: &Config,
    user_repository: Arc<PostgresUserRepository>,
    refresh_token_repository: Arc<PostgresRefreshTokenRepository>,
    email_verification_repository: Arc<PostgresEmailVerificationRepository>,
    password_reset_token_repository: Arc<PostgresPasswordResetTokenRepository>,
) -> Result<Arc<AuthAppState>> {
    let password_hash_service = PasswordHashService::new();
    let jwt_service = JwtService::new(&config.jwt_secret, Some(15), Some(10080));
    let email_service = EmailService::new(
        &config.smtp_host,
        config.smtp_port,
        config.smtp_user.clone(),
        config.smtp_pass.clone(),
        config.smtp_from_email.clone(),
        config.smtp_from_name.clone(),
    )?;
    let google_oauth_service = GoogleOAuthService::new(
        config.google_client_id.clone(),
        config.google_client_secret.clone(),
        config.google_redirect_uri.clone(),
    );
    let two_factor_service = TwoFactorService::new(config.app_name.clone());
    let otp_service = OtpService::new();

    Ok(Arc::new(AuthAppState {
        user_repository,
        refresh_token_repository,
        email_verification_repository,
        password_reset_token_repository,
        password_hash_service,
        jwt_service,
        email_service,
        google_oauth_service,
        two_factor_service,
        otp_service,
        base_url: config.base_url.clone(),
    }))
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;

    let config = load_environment_config()?;
    let db_pool = setup_database_pool(&config.database_url).await?;

    let (
        user_repository,
        account_repository,
        transaction_repository,
        budget_repository,
        refresh_token_repository,
        email_verification_repository,
        password_reset_token_repository,
    ) = setup_repositories(db_pool.clone());

    let auth_state = setup_auth_state(
        &config,
        user_repository.clone(),
        refresh_token_repository,
        email_verification_repository,
        password_reset_token_repository,
    )?;

    let use_cases = init_use_cases(
        user_repository,
        account_repository,
        transaction_repository,
        budget_repository,
    );

    let app = build_router(auth_state, use_cases, db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Server listening on {}", listener.local_addr()?);
    tracing::info!("API Documentation available at http://localhost:3000/docs");

    axum::serve(listener, app).await?;

    Ok(())
}
