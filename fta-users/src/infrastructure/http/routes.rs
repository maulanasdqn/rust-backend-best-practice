use axum::{routing::get, Router};

use super::handlers;

pub fn user_routes() -> Router {
    Router::new()
        .route(
            "/users",
            get(handlers::list_users_handler).post(handlers::create_user_handler),
        )
        .route(
            "/users/{id}",
            get(handlers::get_user_handler)
                .put(handlers::update_user_handler)
                .delete(handlers::delete_user_handler),
        )
}
