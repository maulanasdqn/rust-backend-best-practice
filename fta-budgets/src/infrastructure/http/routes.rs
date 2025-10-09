use super::handlers;
use axum::{
    routing::{get, patch},
    Router,
};

pub fn budget_routes() -> Router {
    Router::new()
        .route(
            "/budgets/{id}",
            get(handlers::get_budget_handler)
                .put(handlers::update_budget_handler)
                .delete(handlers::delete_budget_handler),
        )
        .route(
            "/budgets/{id}/deactivate",
            patch(handlers::deactivate_budget_handler),
        )
        .route(
            "/users/{user_id}/budgets",
            get(handlers::list_budgets_handler).post(handlers::create_budget_handler),
        )
}
