pub mod dto;
pub mod handlers;
mod routes;

pub use dto::{AccountResponse, CreateAccountRequest, UpdateAccountRequest};
pub use handlers::{
    create_account_handler, deactivate_account_handler, delete_account_handler,
    get_account_handler, list_accounts_handler, update_account_handler,
};
pub use routes::account_routes;
