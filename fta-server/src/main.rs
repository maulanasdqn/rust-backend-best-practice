mod api_doc;
mod logger;

use std::sync::Arc;

use anyhow::Result;
use axum::{response::Redirect, routing::get, Extension, Router};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use utoipa::OpenApi;
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

/// Initialize all use cases for the application
#[allow(clippy::type_complexity)]
fn init_use_cases(
    user_repository: Arc<dyn fta_users::domain::UserRepository>,
    account_repository: Arc<PostgresAccountRepository>,
    transaction_repository: Arc<PostgresTransactionRepository>,
    budget_repository: Arc<PostgresBudgetRepository>,
) -> (
    // User use cases
    Arc<GetUser>,
    Arc<ListUsers>,
    Arc<CreateUser>,
    Arc<UpdateUser>,
    Arc<DeleteUser>,
    // Account use cases
    Arc<GetAccount>,
    Arc<ListAccounts>,
    Arc<CreateAccount>,
    Arc<UpdateAccount>,
    Arc<DeleteAccount>,
    Arc<DeactivateAccount>,
    // Transaction use cases
    Arc<GetTransaction>,
    Arc<ListTransactions>,
    Arc<CreateTransaction>,
    Arc<UpdateTransaction>,
    Arc<DeleteTransaction>,
    // Budget use cases
    Arc<GetBudget>,
    Arc<ListBudgets>,
    Arc<CreateBudget>,
    Arc<UpdateBudget>,
    Arc<DeleteBudget>,
    Arc<DeactivateBudget>,
) {
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

    (
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
    )
}

/// Build the application router with all routes and middleware
#[allow(clippy::too_many_arguments)]
fn build_router(
    auth_state: Arc<AuthAppState>,
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
) -> Router {
    let api_v1 = Router::new()
        .merge(auth_routes(auth_state))
        .merge(user_routes())
        .merge(account_routes())
        .merge(transaction_routes())
        .merge(budget_routes())
        .layer(Extension(get_user))
        .layer(Extension(list_users))
        .layer(Extension(create_user))
        .layer(Extension(update_user))
        .layer(Extension(delete_user))
        .layer(Extension(get_account))
        .layer(Extension(list_accounts))
        .layer(Extension(create_account))
        .layer(Extension(update_account))
        .layer(Extension(delete_account))
        .layer(Extension(deactivate_account))
        .layer(Extension(get_transaction))
        .layer(Extension(list_transactions))
        .layer(Extension(create_transaction))
        .layer(Extension(update_transaction))
        .layer(Extension(delete_transaction))
        .layer(Extension(get_budget))
        .layer(Extension(list_budgets))
        .layer(Extension(create_budget))
        .layer(Extension(update_budget))
        .layer(Extension(delete_budget))
        .layer(Extension(deactivate_budget));

    Router::new()
        .route("/", get(redirect_to_docs))
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
    // Initialize logger
    init_logger()?;

    dotenvy::dotenv().ok();
    #[allow(clippy::expect_used)]
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = create_pool(&database_url).await?;
    tracing::info!("Database connection established");

    // Initialize repositories
    let user_repository = Arc::new(PostgresUserRepository::new(db_pool.clone()));
    let account_repository = Arc::new(PostgresAccountRepository::new(db_pool.clone()));
    let transaction_repository = Arc::new(PostgresTransactionRepository::new(db_pool.clone()));
    let budget_repository = Arc::new(PostgresBudgetRepository::new(db_pool.clone()));

    // Initialize auth repositories
    let refresh_token_repository = Arc::new(PostgresRefreshTokenRepository::new(db_pool.clone()));
    let email_verification_repository =
        Arc::new(PostgresEmailVerificationRepository::new(db_pool.clone()));
    let password_reset_token_repository =
        Arc::new(PostgresPasswordResetTokenRepository::new(db_pool.clone()));

    // Initialize auth services
    #[allow(clippy::expect_used)]
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    #[allow(clippy::expect_used)]
    let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let smtp_port: u16 = std::env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".to_string())
        .parse()
        .unwrap_or(587);
    #[allow(clippy::expect_used)]
    let smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER must be set");
    #[allow(clippy::expect_used)]
    let smtp_pass = std::env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    #[allow(clippy::expect_used)]
    let smtp_from_email = std::env::var("SMTP_FROM_EMAIL").expect("SMTP_FROM_EMAIL must be set");
    let smtp_from_name =
        std::env::var("SMTP_FROM_NAME").unwrap_or_else(|_| "Financial Tracker".to_string());
    #[allow(clippy::expect_used)]
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    #[allow(clippy::expect_used)]
    let google_client_secret =
        std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    #[allow(clippy::expect_used)]
    let google_redirect_uri =
        std::env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");
    #[allow(clippy::expect_used)]
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "Financial Tracker".to_string());

    let password_hash_service = PasswordHashService::new();
    let jwt_service = JwtService::new(&jwt_secret, Some(15), Some(10080)); // 15 min access, 7 days refresh
    let email_service = EmailService::new(
        &smtp_host,
        smtp_port,
        smtp_user,
        smtp_pass,
        smtp_from_email,
        smtp_from_name,
    )?;
    let google_oauth_service =
        GoogleOAuthService::new(google_client_id, google_client_secret, google_redirect_uri);
    let two_factor_service = TwoFactorService::new(app_name.clone());
    let otp_service = OtpService::new();

    // Create auth app state
    let auth_state = Arc::new(AuthAppState {
        user_repository: user_repository.clone(),
        refresh_token_repository: refresh_token_repository.clone(),
        email_verification_repository: email_verification_repository.clone(),
        password_reset_token_repository: password_reset_token_repository.clone(),
        password_hash_service: password_hash_service.clone(),
        jwt_service: jwt_service.clone(),
        email_service: email_service.clone(),
        google_oauth_service: google_oauth_service.clone(),
        two_factor_service: two_factor_service.clone(),
        otp_service: otp_service.clone(),
        base_url: base_url.clone(),
    });

    // Initialize all use cases
    let (
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
    ) = init_use_cases(
        user_repository,
        account_repository,
        transaction_repository,
        budget_repository,
    );

    // Build application router
    let app = build_router(
        auth_state,
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
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Server listening on {}", listener.local_addr()?);
    tracing::info!("API Documentation available at http://localhost:3000/docs");

    axum::serve(listener, app).await?;

    Ok(())
}
