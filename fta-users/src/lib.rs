//! # User Management Module
//!
//! This module handles user profile operations following Clean Architecture principles.
//!
//! ## Architecture
//!
//! - **Domain Layer** (`domain/`): Core `User` entity and `UserRepository` trait
//! - **Application Layer** (`application/`): Use cases for user operations
//! - **Infrastructure Layer** (`infrastructure/`): HTTP handlers, DTOs, and PostgreSQL repository
//!
//! ## Use Cases
//!
//! - [`CreateUser`] - Create a new user profile
//! - [`GetUser`] - Retrieve a user by ID
//! - [`ListUsers`] - List users with filtering and pagination
//! - [`UpdateUser`] - Update user profile details
//! - [`DeleteUser`] - Permanently delete a user
//!
//! Note: Authentication-related user operations (login, register, password reset)
//! are handled by the `fta-auth` module.

pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{CreateUser, DeleteUser, GetUser, ListUsers, UpdateUser};
pub use domain::{User, UserRepository};
pub use infrastructure::{
    user_routes, CreateUserRequest, PostgresUserRepository, UpdateUserRequest, UserResponse,
};
