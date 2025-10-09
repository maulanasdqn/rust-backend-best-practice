pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types
pub use application::{CreateUser, DeleteUser, GetUser, ListUsers, UpdateUser};
pub use domain::{User, UserRepository};
pub use infrastructure::{
    user_routes, CreateUserRequest, PostgresUserRepository, UpdateUserRequest, UserResponse,
};
