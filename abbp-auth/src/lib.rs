//! # Authentication & Authorization Module
//!
//! This module handles user authentication, authorization, and security features
//! following Clean Architecture principles.
//!
//! ## Architecture
//!
//! - **Domain Layer** (`domain/`): Token entities and repository traits
//! - **Application Layer** (`application/`): Authentication use cases
//! - **Infrastructure Layer** (`infrastructure/`): HTTP handlers, JWT service, password hashing, OAuth, etc.
//!
//! ## Authentication Use Cases
//!
//! - [`Login`] - Authenticate with email/password
//! - [`RegisterUser`] - Create a new user account
//! - [`RefreshAccessToken`] - Get a new access token using refresh token
//! - [`Logout`] / [`LogoutAll`] - Revoke refresh tokens
//!
//! ## Password Management
//!
//! - [`ChangePassword`] - Change password (requires current password)
//! - [`RequestPasswordReset`] - Send password reset email
//! - [`ResetPassword`] - Reset password using token
//!
//! ## Two-Factor Authentication (2FA)
//!
//! - [`Enable2FA`] - Generate 2FA setup (secret, QR code)
//! - [`Verify2FA`] - Verify 2FA code
//! - [`Disable2FA`] - Disable 2FA (requires password + code)
//!
//! ## OAuth
//!
//! - [`GoogleOAuthLogin`] - Get Google OAuth URL
//! - [`GoogleOAuthCallback`] - Handle OAuth callback
//!
//! ## Email Verification
//!
//! - [`VerifyEmail`] - Verify email with OTP code

pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::*;
pub use infrastructure::{
    http::{auth_routes, handlers::AuthAppState, middleware::*},
    persistence::*,
    services::*,
};
