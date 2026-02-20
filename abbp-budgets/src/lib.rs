//! # Budget Management Module
//!
//! This module handles budget tracking operations following Clean Architecture principles.
//!
//! ## Architecture
//!
//! - **Domain Layer** (`domain/`): Core `Budget` entity, `BudgetPeriod` enum, and repository trait
//! - **Application Layer** (`application/`): Use cases for budget operations
//! - **Infrastructure Layer** (`infrastructure/`): HTTP handlers, DTOs, and PostgreSQL repository
//!
//! ## Use Cases
//!
//! - [`CreateBudget`] - Create a new budget for a category
//! - [`GetBudget`] - Retrieve a budget by ID
//! - [`ListBudgets`] - List budgets with filtering and pagination
//! - [`UpdateBudget`] - Update budget amount or category
//! - [`DeleteBudget`] - Permanently delete a budget
//! - [`DeactivateBudget`] - End a budget by setting an end date
//!
//! ## Budget Periods
//!
//! Budgets can be set for different periods: `Daily`, `Weekly`, `Monthly`, or `Yearly`.

pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{
    CreateBudget, DeactivateBudget, DeleteBudget, GetBudget, ListBudgets, UpdateBudget,
};
pub use domain::{Budget, BudgetPeriod, BudgetRepository};
pub use infrastructure::{
    budget_routes, BudgetResponse, CreateBudgetRequest, PostgresBudgetRepository,
    UpdateBudgetRequest,
};
