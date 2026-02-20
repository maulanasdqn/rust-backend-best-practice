//! Application router configuration with middleware layers.

use std::sync::Arc;

use axum::{response::Redirect, routing::get, Extension, Router};
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use utoipa_swagger_ui::SwaggerUi;

use abbp_accounts::account_routes;
use abbp_auth::{auth_routes, AuthAppState};
use abbp_budgets::budget_routes;
use abbp_transactions::transaction_routes;
use abbp_users::user_routes;

use crate::api_doc::ApiDoc;
use crate::health;
use crate::use_cases::UseCases;

/// Maximum request body size in bytes (1 MB).
const MAX_BODY_SIZE: usize = 1024 * 1024;

/// Rate limit: requests per second per IP.
const RATE_LIMIT_PER_SECOND: u64 = 50;

/// Rate limit burst size (max requests that can be made instantly).
const RATE_LIMIT_BURST: u32 = 100;

async fn redirect_to_docs() -> Redirect {
    Redirect::permanent("/docs")
}

pub fn build_router(
    auth_state: Arc<AuthAppState>,
    use_cases: UseCases,
    db_pool: abbp_database::DbPool,
) -> Router {
    // Configure rate limiting per IP address
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(RATE_LIMIT_PER_SECOND)
            .burst_size(RATE_LIMIT_BURST)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .expect("Failed to build rate limiter config"),
    );

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
        // Rate limiting layer (must be before other layers to protect against DoS)
        .layer(GovernorLayer::new(governor_conf))
        // Request body size limit (1 MB)
        .layer(RequestBodyLimitLayer::new(MAX_BODY_SIZE))
        // HTTP request/response tracing
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
        // CORS configuration
        .layer(CorsLayer::permissive())
}
