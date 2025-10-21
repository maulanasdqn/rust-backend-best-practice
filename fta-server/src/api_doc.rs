use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        fta_auth::infrastructure::http::handlers::register_handler,
        fta_auth::infrastructure::http::handlers::verify_email_handler,
        fta_auth::infrastructure::http::handlers::login_handler,
        fta_auth::infrastructure::http::handlers::refresh_token_handler,
        fta_auth::infrastructure::http::handlers::logout_handler,
        fta_auth::infrastructure::http::handlers::logout_all_handler,
        fta_auth::infrastructure::http::handlers::request_password_reset_handler,
        fta_auth::infrastructure::http::handlers::reset_password_handler,
        fta_auth::infrastructure::http::handlers::change_password_handler,
        fta_auth::infrastructure::http::handlers::enable_2fa_handler,
        fta_auth::infrastructure::http::handlers::verify_2fa_handler,
        fta_auth::infrastructure::http::handlers::disable_2fa_handler,
        fta_auth::infrastructure::http::handlers::google_oauth_login_handler,
        fta_auth::infrastructure::http::handlers::google_oauth_callback_handler,
    ),
    components(
        schemas(
            fta_auth::infrastructure::http::dto::RegisterRequest,
            fta_auth::infrastructure::http::dto::RegisterResponse,
            fta_auth::infrastructure::http::dto::VerifyEmailRequest,
            fta_auth::infrastructure::http::dto::LoginRequest,
            fta_auth::infrastructure::http::dto::LoginResponse,
            fta_auth::infrastructure::http::dto::RefreshTokenRequest,
            fta_auth::infrastructure::http::dto::RefreshTokenResponse,
            fta_auth::infrastructure::http::dto::LogoutRequest,
            fta_auth::infrastructure::http::dto::RequestPasswordResetRequest,
            fta_auth::infrastructure::http::dto::ResetPasswordRequest,
            fta_auth::infrastructure::http::dto::ChangePasswordRequest,
            fta_auth::infrastructure::http::dto::Enable2FAResponse,
            fta_auth::infrastructure::http::dto::Verify2FARequest,
            fta_auth::infrastructure::http::dto::Disable2FARequest,
            fta_auth::infrastructure::http::dto::GoogleOAuthCallbackRequest,
            fta_auth::infrastructure::http::dto::GoogleOAuthCallbackResponse,
            fta_auth::infrastructure::http::dto::MessageResponse,
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
        fta_users::infrastructure::http::handlers::get_user_handler,
        fta_users::infrastructure::http::handlers::list_users_handler,
        fta_users::infrastructure::http::handlers::create_user_handler,
        fta_users::infrastructure::http::handlers::update_user_handler,
        fta_users::infrastructure::http::handlers::delete_user_handler,
    ),
    components(
        schemas(
            fta_users::domain::User,
            fta_users::UserResponse,
            fta_users::CreateUserRequest,
            fta_users::UpdateUserRequest,
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
        fta_accounts::infrastructure::http::handlers::get_account_handler,
        fta_accounts::infrastructure::http::handlers::list_accounts_handler,
        fta_accounts::infrastructure::http::handlers::create_account_handler,
        fta_accounts::infrastructure::http::handlers::update_account_handler,
        fta_accounts::infrastructure::http::handlers::delete_account_handler,
        fta_accounts::infrastructure::http::handlers::deactivate_account_handler,
    ),
    components(
        schemas(
            fta_accounts::domain::Account,
            fta_accounts::domain::AccountType,
            fta_accounts::AccountResponse,
            fta_accounts::CreateAccountRequest,
            fta_accounts::UpdateAccountRequest,
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
        fta_transactions::infrastructure::http::handlers::get_transaction_handler,
        fta_transactions::infrastructure::http::handlers::list_transactions_handler,
        fta_transactions::infrastructure::http::handlers::create_transaction_handler,
        fta_transactions::infrastructure::http::handlers::update_transaction_handler,
        fta_transactions::infrastructure::http::handlers::delete_transaction_handler,
    ),
    components(
        schemas(
            fta_transactions::domain::Transaction,
            fta_transactions::domain::TransactionType,
            fta_transactions::TransactionResponse,
            fta_transactions::CreateTransactionRequest,
            fta_transactions::UpdateTransactionRequest,
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
        fta_budgets::infrastructure::http::handlers::get_budget_handler,
        fta_budgets::infrastructure::http::handlers::list_budgets_handler,
        fta_budgets::infrastructure::http::handlers::create_budget_handler,
        fta_budgets::infrastructure::http::handlers::update_budget_handler,
        fta_budgets::infrastructure::http::handlers::delete_budget_handler,
        fta_budgets::infrastructure::http::handlers::deactivate_budget_handler,
    ),
    components(
        schemas(
            fta_budgets::domain::Budget,
            fta_budgets::domain::BudgetPeriod,
            fta_budgets::BudgetResponse,
            fta_budgets::CreateBudgetRequest,
            fta_budgets::UpdateBudgetRequest,
        )
    ),
    tags(
        (name = "Budgets", description = "Budget management endpoints"),
    )
)]
struct BudgetsApiDoc;

#[derive(OpenApi)]
#[openapi(components(schemas(
    fta_types::Money,
    fta_types::DateRange,
    fta_types::ErrorResponse,
    fta_types::PaginationMeta,
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
