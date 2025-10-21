mod api_doc;
mod health;
mod logger;

use std::sync::Arc;

use anyhow::Result;
use axum::{response::Redirect, routing::get, Extension, Router};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use utoipa_swagger_ui::SwaggerUi;

use fta_accounts::{
    account_routes, CreateAccount, DeactivateAccount, DeleteAccount, GetAccount, ListAccounts,
    PostgresAccountRepository, UpdateAccount,
};
use fta_auth::{
    auth_routes, AuthAppState, EmailService, GoogleOAuthService, JwtService, OtpService,
    PasswordHashService, PostgresEmailVerificationRepository, PostgresPasswordResetTokenRepository,
    PostgresRefreshTokenRepository, TwoFactorService,
};
use fta_budgets::{
    budget_routes, CreateBudget, DeactivateBudget, DeleteBudget, GetBudget, ListBudgets,
    PostgresBudgetRepository, UpdateBudget,
};
use fta_database::create_pool;
use fta_transactions::{
    transaction_routes, CreateTransaction, DeleteTransaction, GetTransaction, ListTransactions,
    PostgresTransactionRepository, UpdateTransaction,
};
use fta_users::{
    user_routes, CreateUser, DeleteUser, GetUser, ListUsers, PostgresUserRepository, UpdateUser,
};

use api_doc::ApiDoc;
use logger::init_logger;

async fn redirect_to_docs() -> Redirect {
    Redirect::permanent("/docs")
}

struct Config {
    database_url: String,
    jwt_secret: String,
    smtp_host: String,
    smtp_port: u16,
    smtp_user: String,
    smtp_pass: String,
    smtp_from_email: String,
    smtp_from_name: String,
    google_client_id: String,
    google_client_secret: String,
    google_redirect_uri: String,
    base_url: String,
    app_name: String,
}

fn load_environment_config() -> Result<Config> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let smtp_host = std::env::var("SMTP_HOST")?;
    let smtp_port: u16 = std::env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".to_string())
        .parse()
        .unwrap_or(587);
    let smtp_user = std::env::var("SMTP_USER")?;
    let smtp_pass = std::env::var("SMTP_PASS")?;
    let smtp_from_email = std::env::var("SMTP_FROM_EMAIL")?;
    let smtp_from_name =
        std::env::var("SMTP_FROM_NAME").unwrap_or_else(|_| "Financial Tracker".to_string());
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID")?;
    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET")?;
    let google_redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")?;
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "Financial Tracker".to_string());

    Ok(Config {
        database_url,
        jwt_secret,
        smtp_host,
        smtp_port,
        smtp_user,
        smtp_pass,
        smtp_from_email,
        smtp_from_name,
        google_client_id,
        google_client_secret,
        google_redirect_uri,
        base_url,
        app_name,
    })
}

async fn setup_database_pool(database_url: &str) -> Result<fta_database::DbPool> {
    let db_pool = create_pool(database_url).await?;
    tracing::info!("Database connection established");
    Ok(db_pool)
}

type Repositories = (
    Arc<PostgresUserRepository>,
    Arc<PostgresAccountRepository>,
    Arc<PostgresTransactionRepository>,
    Arc<PostgresBudgetRepository>,
    Arc<PostgresRefreshTokenRepository>,
    Arc<PostgresEmailVerificationRepository>,
    Arc<PostgresPasswordResetTokenRepository>,
);

fn setup_repositories(db_pool: fta_database::DbPool) -> Repositories {
    let user_repository = Arc::new(PostgresUserRepository::new(db_pool.clone()));
    let account_repository = Arc::new(PostgresAccountRepository::new(db_pool.clone()));
    let transaction_repository = Arc::new(PostgresTransactionRepository::new(db_pool.clone()));
    let budget_repository = Arc::new(PostgresBudgetRepository::new(db_pool.clone()));
    let refresh_token_repository = Arc::new(PostgresRefreshTokenRepository::new(db_pool.clone()));
    let email_verification_repository =
        Arc::new(PostgresEmailVerificationRepository::new(db_pool.clone()));
    let password_reset_token_repository =
        Arc::new(PostgresPasswordResetTokenRepository::new(db_pool));

    (
        user_repository,
        account_repository,
        transaction_repository,
        budget_repository,
        refresh_token_repository,
        email_verification_repository,
        password_reset_token_repository,
    )
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

struct UseCases {
    get_user: Arc<GetUser>,
    list_users: Arc<ListUsers>,
    create_user: Arc<CreateUser>,
    update_user: Arc<UpdateUser>,
    delete_user: Arc<DeleteUser>,
    get_account: Arc<GetAccount>,
    list_accounts: Arc<ListAccounts>,
    create_account: Arc<CreateAccount>,
    update_account: Arc<UpdateAccount>,
    delete_account: Arc<DeleteAccount>,
    deactivate_account: Arc<DeactivateAccount>,
    get_transaction: Arc<GetTransaction>,
    list_transactions: Arc<ListTransactions>,
    create_transaction: Arc<CreateTransaction>,
    update_transaction: Arc<UpdateTransaction>,
    delete_transaction: Arc<DeleteTransaction>,
    get_budget: Arc<GetBudget>,
    list_budgets: Arc<ListBudgets>,
    create_budget: Arc<CreateBudget>,
    update_budget: Arc<UpdateBudget>,
    delete_budget: Arc<DeleteBudget>,
    deactivate_budget: Arc<DeactivateBudget>,
}

fn init_use_cases(
    user_repository: Arc<dyn fta_users::domain::UserRepository>,
    account_repository: Arc<PostgresAccountRepository>,
    transaction_repository: Arc<PostgresTransactionRepository>,
    budget_repository: Arc<PostgresBudgetRepository>,
) -> UseCases {
    let get_user = Arc::new(GetUser::new(user_repository.clone()));
    let list_users = Arc::new(ListUsers::new(user_repository.clone()));
    let create_user = Arc::new(CreateUser::new(user_repository.clone()));
    let update_user = Arc::new(UpdateUser::new(user_repository.clone()));
    let delete_user = Arc::new(DeleteUser::new(user_repository));

    let get_account = Arc::new(GetAccount::new(account_repository.clone()));
    let list_accounts = Arc::new(ListAccounts::new(account_repository.clone()));
    let create_account = Arc::new(CreateAccount::new(account_repository.clone()));
    let update_account = Arc::new(UpdateAccount::new(account_repository.clone()));
    let delete_account = Arc::new(DeleteAccount::new(account_repository.clone()));
    let deactivate_account = Arc::new(DeactivateAccount::new(account_repository));

    let get_transaction = Arc::new(GetTransaction::new(transaction_repository.clone()));
    let list_transactions = Arc::new(ListTransactions::new(transaction_repository.clone()));
    let create_transaction = Arc::new(CreateTransaction::new(transaction_repository.clone()));
    let update_transaction = Arc::new(UpdateTransaction::new(transaction_repository.clone()));
    let delete_transaction = Arc::new(DeleteTransaction::new(transaction_repository));

    let get_budget = Arc::new(GetBudget::new(budget_repository.clone()));
    let list_budgets = Arc::new(ListBudgets::new(budget_repository.clone()));
    let create_budget = Arc::new(CreateBudget::new(budget_repository.clone()));
    let update_budget = Arc::new(UpdateBudget::new(budget_repository.clone()));
    let delete_budget = Arc::new(DeleteBudget::new(budget_repository.clone()));
    let deactivate_budget = Arc::new(DeactivateBudget::new(budget_repository));

    UseCases {
        get_user,
        list_users,
        create_user,
        update_user,
        delete_user,
        get_account,
        list_accounts,
        create_account,
        update_account,
        delete_account,
        deactivate_account,
        get_transaction,
        list_transactions,
        create_transaction,
        update_transaction,
        delete_transaction,
        get_budget,
        list_budgets,
        create_budget,
        update_budget,
        delete_budget,
        deactivate_budget,
    }
}

fn build_router(
    auth_state: Arc<AuthAppState>,
    use_cases: UseCases,
    db_pool: fta_database::DbPool,
) -> Router {
    let api_v1 = Router::new()
        .merge(auth_routes(auth_state))
        .merge(user_routes())
        .merge(account_routes())
        .merge(transaction_routes())
        .merge(budget_routes())
        .layer(Extension(use_cases.get_user))
        .layer(Extension(use_cases.list_users))
        .layer(Extension(use_cases.create_user))
        .layer(Extension(use_cases.update_user))
        .layer(Extension(use_cases.delete_user))
        .layer(Extension(use_cases.get_account))
        .layer(Extension(use_cases.list_accounts))
        .layer(Extension(use_cases.create_account))
        .layer(Extension(use_cases.update_account))
        .layer(Extension(use_cases.delete_account))
        .layer(Extension(use_cases.deactivate_account))
        .layer(Extension(use_cases.get_transaction))
        .layer(Extension(use_cases.list_transactions))
        .layer(Extension(use_cases.create_transaction))
        .layer(Extension(use_cases.update_transaction))
        .layer(Extension(use_cases.delete_transaction))
        .layer(Extension(use_cases.get_budget))
        .layer(Extension(use_cases.list_budgets))
        .layer(Extension(use_cases.create_budget))
        .layer(Extension(use_cases.update_budget))
        .layer(Extension(use_cases.delete_budget))
        .layer(Extension(use_cases.deactivate_budget));

    Router::new()
        .route("/", get(redirect_to_docs))
        .route("/health", get(health::health_check))
        .route("/ready", get(health::readiness_check))
        .with_state(Arc::new(db_pool))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", api_v1)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .include_headers(true)
                        .level(tracing::Level::INFO),
                )
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .latency_unit(LatencyUnit::Millis)
                        .level(tracing::Level::INFO),
                ),
        )
        .layer(CorsLayer::permissive())
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
