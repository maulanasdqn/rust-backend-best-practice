use std::sync::Arc;

use axum::{response::Redirect, routing::get, Extension, Router};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use utoipa_swagger_ui::SwaggerUi;

use fta_accounts::account_routes;
use fta_auth::{auth_routes, AuthAppState};
use fta_budgets::budget_routes;
use fta_transactions::transaction_routes;
use fta_users::user_routes;

use crate::api_doc::ApiDoc;
use crate::health;
use crate::use_cases::UseCases;

async fn redirect_to_docs() -> Redirect {
    Redirect::permanent("/docs")
}

pub fn build_router(
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
