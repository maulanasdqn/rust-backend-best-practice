use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        abbp_auth::infrastructure::http::handlers::register_handler,
        abbp_auth::infrastructure::http::handlers::verify_email_handler,
        abbp_auth::infrastructure::http::handlers::login_handler,
        abbp_auth::infrastructure::http::handlers::refresh_token_handler,
        abbp_auth::infrastructure::http::handlers::logout_handler,
        abbp_auth::infrastructure::http::handlers::logout_all_handler,
        abbp_auth::infrastructure::http::handlers::request_password_reset_handler,
        abbp_auth::infrastructure::http::handlers::reset_password_handler,
        abbp_auth::infrastructure::http::handlers::change_password_handler,
        abbp_auth::infrastructure::http::handlers::enable_2fa_handler,
        abbp_auth::infrastructure::http::handlers::verify_2fa_handler,
        abbp_auth::infrastructure::http::handlers::disable_2fa_handler,
        abbp_auth::infrastructure::http::handlers::google_oauth_login_handler,
        abbp_auth::infrastructure::http::handlers::google_oauth_callback_handler,
    ),
    components(
        schemas(
            abbp_auth::infrastructure::http::dto::RegisterRequest,
            abbp_auth::infrastructure::http::dto::RegisterData,
            abbp_auth::infrastructure::http::dto::VerifyEmailRequest,
            abbp_auth::infrastructure::http::dto::LoginRequest,
            abbp_auth::infrastructure::http::dto::TokenData,
            abbp_auth::infrastructure::http::dto::LoginData,
            abbp_auth::infrastructure::http::dto::RefreshTokenRequest,
            abbp_auth::infrastructure::http::dto::RefreshTokenData,
            abbp_auth::infrastructure::http::dto::LogoutRequest,
            abbp_auth::infrastructure::http::dto::RequestPasswordResetRequest,
            abbp_auth::infrastructure::http::dto::ResetPasswordRequest,
            abbp_auth::infrastructure::http::dto::ChangePasswordRequest,
            abbp_auth::infrastructure::http::dto::Enable2FAData,
            abbp_auth::infrastructure::http::dto::Verify2FARequest,
            abbp_auth::infrastructure::http::dto::Disable2FARequest,
            abbp_auth::infrastructure::http::dto::GoogleOAuthCallbackRequest,
            abbp_auth::infrastructure::http::dto::GoogleOAuthData,
            abbp_auth::infrastructure::http::handlers::GoogleOAuthLoginData,
        )
    ),
    tags(
        (name = "Authentication", description = "Authentication and authorization endpoints"),
        (name = "Two-Factor Authentication", description = "Two-factor authentication (2FA) endpoints"),
        (name = "OAuth", description = "OAuth 2.0 authentication endpoints"),
    )
)]
struct AuthApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        abbp_users::infrastructure::http::handlers::get_user_handler,
        abbp_users::infrastructure::http::handlers::list_users_handler,
        abbp_users::infrastructure::http::handlers::create_user_handler,
        abbp_users::infrastructure::http::handlers::update_user_handler,
        abbp_users::infrastructure::http::handlers::delete_user_handler,
    ),
    components(
        schemas(
            abbp_users::domain::User,
            abbp_users::UserResponse,
            abbp_users::CreateUserRequest,
            abbp_users::UpdateUserRequest,
        )
    ),
    tags(
        (name = "Users", description = "User management endpoints"),
    )
)]
struct UsersApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        abbp_accounts::infrastructure::http::handlers::get_account_handler,
        abbp_accounts::infrastructure::http::handlers::list_accounts_handler,
        abbp_accounts::infrastructure::http::handlers::create_account_handler,
        abbp_accounts::infrastructure::http::handlers::update_account_handler,
        abbp_accounts::infrastructure::http::handlers::delete_account_handler,
        abbp_accounts::infrastructure::http::handlers::deactivate_account_handler,
    ),
    components(
        schemas(
            abbp_accounts::domain::Account,
            abbp_accounts::domain::AccountType,
            abbp_accounts::AccountResponse,
            abbp_accounts::CreateAccountRequest,
            abbp_accounts::UpdateAccountRequest,
        )
    ),
    tags(
        (name = "Accounts", description = "Account management endpoints"),
    )
)]
struct AccountsApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        abbp_transactions::infrastructure::http::handlers::get_transaction_handler,
        abbp_transactions::infrastructure::http::handlers::list_transactions_handler,
        abbp_transactions::infrastructure::http::handlers::create_transaction_handler,
        abbp_transactions::infrastructure::http::handlers::update_transaction_handler,
        abbp_transactions::infrastructure::http::handlers::delete_transaction_handler,
    ),
    components(
        schemas(
            abbp_transactions::domain::Transaction,
            abbp_transactions::domain::TransactionType,
            abbp_transactions::TransactionResponse,
            abbp_transactions::CreateTransactionRequest,
            abbp_transactions::UpdateTransactionRequest,
        )
    ),
    tags(
        (name = "Transactions", description = "Transaction management endpoints"),
    )
)]
struct TransactionsApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(
        abbp_budgets::infrastructure::http::handlers::get_budget_handler,
        abbp_budgets::infrastructure::http::handlers::list_budgets_handler,
        abbp_budgets::infrastructure::http::handlers::create_budget_handler,
        abbp_budgets::infrastructure::http::handlers::update_budget_handler,
        abbp_budgets::infrastructure::http::handlers::delete_budget_handler,
        abbp_budgets::infrastructure::http::handlers::deactivate_budget_handler,
    ),
    components(
        schemas(
            abbp_budgets::domain::Budget,
            abbp_budgets::domain::BudgetPeriod,
            abbp_budgets::BudgetResponse,
            abbp_budgets::CreateBudgetRequest,
            abbp_budgets::UpdateBudgetRequest,
        )
    ),
    tags(
        (name = "Budgets", description = "Budget management endpoints"),
    )
)]
struct BudgetsApiDoc;

#[derive(OpenApi)]
#[openapi(components(schemas(
    abbp_types::Money,
    abbp_types::DateRange,
    abbp_types::ErrorResponse,
    abbp_types::MessageOnlyResponse,
    abbp_types::PaginationMeta,
)))]
struct CommonTypesApiDoc;

pub struct ApiDoc;

impl ApiDoc {
    pub fn openapi() -> utoipa::openapi::OpenApi {
        use utoipa::openapi::{InfoBuilder, OpenApiBuilder};

        let info = InfoBuilder::new()
            .title("Financial Tracker API")
            .version("0.1.0")
            .description(Some("A comprehensive financial tracking API for managing users, accounts, transactions, and budgets"))
            .build();

        let mut api = OpenApiBuilder::new().info(info).build();

        api.merge(AuthApiDoc::openapi());
        api.merge(UsersApiDoc::openapi());
        api.merge(AccountsApiDoc::openapi());
        api.merge(TransactionsApiDoc::openapi());
        api.merge(BudgetsApiDoc::openapi());
        api.merge(CommonTypesApiDoc::openapi());

        api
    }
}
