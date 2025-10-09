use axum::{routing::get, Router};

use super::handlers;

pub fn transaction_routes() -> Router {
    Router::new()
        .route(
            "/transactions/{id}",
            get(handlers::get_transaction_handler)
                .put(handlers::update_transaction_handler)
                .delete(handlers::delete_transaction_handler),
        )
        .route(
            "/accounts/{account_id}/transactions",
            get(handlers::list_transactions_handler).post(handlers::create_transaction_handler),
        )
}
