//! # Account Management Module
//!
//! This module handles financial account operations following Clean Architecture principles.
//!
//! ## Architecture
//!
//! - **Domain Layer** (`domain/`): Core `Account` entity and `AccountRepository` trait
//! - **Application Layer** (`application/`): Use cases for account operations
//! - **Infrastructure Layer** (`infrastructure/`): HTTP handlers, DTOs, and PostgreSQL repository
//!
//! ## Use Cases
//!
//! - [`CreateAccount`] - Create a new financial account
//! - [`GetAccount`] - Retrieve an account by ID
//! - [`ListAccounts`] - List accounts with filtering and pagination
//! - [`UpdateAccount`] - Update account details
//! - [`DeleteAccount`] - Permanently delete an account
//! - [`DeactivateAccount`] - Soft-delete by setting an end date

pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{
    CreateAccount, DeactivateAccount, DeleteAccount, GetAccount, ListAccounts, UpdateAccount,
};
pub use domain::{Account, AccountRepository, AccountType};
pub use infrastructure::{
    account_routes, AccountResponse, CreateAccountRequest, PostgresAccountRepository,
    UpdateAccountRequest,
};
