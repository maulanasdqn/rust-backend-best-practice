//! # Transaction Management Module
//!
//! This module handles financial transaction operations following Clean Architecture principles.
//!
//! ## Architecture
//!
//! - **Domain Layer** (`domain/`): Core `Transaction` entity, `TransactionType` enum, and repository trait
//! - **Application Layer** (`application/`): Use cases for transaction operations
//! - **Infrastructure Layer** (`infrastructure/`): HTTP handlers, DTOs, and PostgreSQL repository
//!
//! ## Use Cases
//!
//! - [`CreateTransaction`] - Record a new financial transaction
//! - [`GetTransaction`] - Retrieve a transaction by ID
//! - [`ListTransactions`] - List transactions with filtering and pagination
//! - [`UpdateTransaction`] - Update transaction details
//! - [`DeleteTransaction`] - Permanently delete a transaction
//!
//! ## Transaction Types
//!
//! Transactions can be of type `Income`, `Expense`, or `Transfer` (between accounts).

pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{
    CreateTransaction, DeleteTransaction, GetTransaction, ListTransactions, UpdateTransaction,
};
pub use domain::{Transaction, TransactionRepository, TransactionType};
pub use infrastructure::{
    transaction_routes, CreateTransactionRequest, PostgresTransactionRepository,
    TransactionResponse, UpdateTransactionRequest,
};
