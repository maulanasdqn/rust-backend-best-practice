use axum::{
    routing::{get, patch},
    Router,
};

use super::handlers;

pub fn account_routes() -> Router {
    Router::new()
        .route(
            "/accounts/{id}",
            get(handlers::get_account_handler)
                .put(handlers::update_account_handler)
                .delete(handlers::delete_account_handler),
        )
        .route(
            "/accounts/{id}/deactivate",
            patch(handlers::deactivate_account_handler),
        )
        .route(
            "/users/{user_id}/accounts",
            get(handlers::list_accounts_handler).post(handlers::create_account_handler),
        )
}
