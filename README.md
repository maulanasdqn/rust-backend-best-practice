# Rust Backend Best Practices

> A comprehensive guide to building production-ready Rust backend services using Clean Architecture, Domain-Driven Design, and modern Rust patterns.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Core Principles](#core-principles)
- [Design Patterns](#design-patterns)
- [Database Patterns](#database-patterns)
- [Validation Strategy](#validation-strategy)
- [Error Handling](#error-handling)
- [Authentication & Authorization](#authentication--authorization)
- [API Design](#api-design)
- [Testing Strategy](#testing-strategy)
- [Development Workflow](#development-workflow)
- [Common Commands](#common-commands)
- [Environment Configuration](#environment-configuration)
- [Best Practices Summary](#best-practices-summary)

## Overview

Financial Tracker API is a production-ready Rust backend service demonstrating enterprise-level patterns and practices. The project showcases:

- **Clean Architecture** with clear separation of concerns
- **Domain-Driven Design** principles
- **Type-safe validation** using zod-rs
- **Compile-time SQL verification** with sqlx
- **JWT-based authentication** with refresh tokens
- **Comprehensive error handling** with custom error types
- **RESTful API** with OpenAPI documentation

## Architecture

### Clean Architecture Layers

```
┌────────────────────────────────────────────────────────────┐
│                     Infrastructure Layer                   │
│  (HTTP Handlers, Repositories, External Services, DTOs)    │
│                                                            │
│  ┌────────────────────────────────────────────────────┐    │
│  │              Application Layer                     │    │
│  │        (Use Cases, Business Logic)                 │    │
│  │                                                    │    │
│  │  ┌──────────────────────────────────────────────┐  │    │
│  │  │           Domain Layer                       │  │    │
│  │  │  (Entities, Value Objects, Domain Traits)    │  │    │
│  │  └──────────────────────────────────────────────┘  │    │
│  │                                                    │    │
│  └────────────────────────────────────────────────────┘    │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

#### 1. Domain Layer (Innermost)

**Purpose**: Contains pure business logic, independent of frameworks and external concerns.

**Components**:
- **Entities**: Core business objects (User, Account, Transaction, Budget)
- **Value Objects**: Immutable objects representing domain concepts (AccountType, TransactionType)
- **Domain Traits**: Repository interfaces defining contracts

**Rules**:
- No dependencies on outer layers
- No external crate dependencies (except serde, uuid, chrono)
- Pure Rust logic only

**Example**:
```rust
// fta-users/src/domain/user.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// fta-users/src/domain/repository.rs
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn create(&self, user: User) -> Result<User>;
    async fn update(&self, user: User) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
```

#### 2. Application Layer (Middle)

**Purpose**: Implements use cases and orchestrates business logic.

**Components**:
- **Use Cases**: Single-responsibility operations (CreateUser, GetAccount, etc.)
- **Application Services**: Coordinate multiple use cases

**Rules**:
- Depends on domain layer only
- No knowledge of HTTP, databases, or external services
- Uses repository traits (not implementations)

**Example**:
```rust
// fta-users/src/application/create_user.rs
pub struct CreateUser {
    repository: Arc<dyn UserRepository>,
}

impl CreateUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user: User) -> Result<User> {
        // Business logic
        if let Some(_existing) = self.repository.find_by_email(&user.email).await? {
            return Err(Error::Conflict("Email already exists".to_string()));
        }

        self.repository.create(user).await
    }
}
```

#### 3. Infrastructure Layer (Outermost)

**Purpose**: Implements technical details and external integrations.

**Components**:
- **HTTP Handlers**: Axum request handlers
- **DTOs**: Request/Response data transfer objects
- **Repository Implementations**: PostgreSQL implementations using sqlx
- **External Services**: Email, OAuth, JWT services
- **Middleware**: Authentication, logging, error handling

**Rules**:
- Depends on application and domain layers
- Contains framework-specific code
- Implements repository traits defined in domain

**Example**:
```rust
// fta-users/src/infrastructure/persistence/postgres_user_repository.rs
pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;

        Ok(user)
    }
}

// fta-users/src/infrastructure/http/handlers.rs
pub async fn create_user_handler(
    State(state): State<Arc<CreateUser>>,
    Validated(req): Validated<CreateUserRequest>,
) -> Result<impl IntoResponse> {
    let user = User::new(req.email, req.password_hash);
    let created = state.execute(user).await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(created))))
}
```

### Dependency Flow

```
HTTP Request
    ↓
Handler (Infrastructure)
    ↓
Use Case (Application)
    ↓
Repository Trait (Domain)
    ↑
Repository Impl (Infrastructure)
    ↓
Database
```

## Technology Stack

### Core Framework
- **Axum 0.8** - Web framework built on tokio and hyper
- **Tokio** - Async runtime
- **Tower** - Middleware and service abstractions

### Database
- **PostgreSQL 17** - Primary database
- **SQLx 0.8** - Compile-time verified SQL queries
- **Custom Migration Tool** - fta-migration for schema management

### Validation & Serialization
- **zod-rs** - Schema-based validation inspired by Zod (TypeScript)
- **Serde** - Serialization/deserialization

### Authentication
- **jsonwebtoken** - JWT token generation/validation
- **Argon2** - Password hashing (OWASP recommended)
- **TOTP-lite** - Two-factor authentication
- **OAuth2** - Google OAuth integration

### Documentation
- **utoipa** - OpenAPI 3.0 documentation
- **utoipa-swagger-ui** - Interactive API documentation

### Development
- **cargo-make** - Task runner (Makefile.toml)
- **dotenvy** - Environment variable management
- **tracing** - Structured logging

## Project Structure

```
financial-tracker-api/
├── fta-auth/              # Authentication module
│   ├── src/
│   │   ├── domain/        # Domain entities and traits
│   │   ├── application/   # Use cases
│   │   └── infrastructure/
│   │       ├── http/      # Handlers, DTOs, routes
│   │       ├── persistence/ # Repository implementations
│   │       └── services/  # External services
│   └── Cargo.toml
│
├── fta-users/             # User management module
│   └── src/
│       ├── domain/
│       ├── application/
│       └── infrastructure/
│
├── fta-accounts/          # Account management module
├── fta-transactions/      # Transaction management module
├── fta-budgets/           # Budget management module
│
├── fta-database/          # Database connection pool
├── fta-errors/            # Centralized error types
├── fta-types/             # Shared types and utilities
├── fta-validation/        # Validation framework
│
├── fta-migration/         # Database migration tool
│   └── migrations/        # SQL migration files
│       ├── 20250101000001_create_users_table.sql
│       ├── 20250101000002_create_accounts_table.sql
│       └── ...
│
├── fta-server/            # Main application entry point
│   └── src/
│       ├── main.rs        # Server setup and routing
│       ├── api_doc.rs     # OpenAPI configuration
│       └── logger.rs      # Logging configuration
│
├── Cargo.toml             # Workspace configuration
├── Makefile.toml          # Cargo-make tasks
├── .env.example           # Environment template
└── README.md
```

### Module Organization Pattern

Each business module follows this structure:

```
fta-{module}/
├── src/
│   ├── domain/
│   │   ├── mod.rs                # Re-exports
│   │   ├── {entity}.rs           # Core entity
│   │   └── repository.rs         # Repository trait
│   │
│   ├── application/
│   │   ├── mod.rs                # Re-exports
│   │   ├── create_{entity}.rs    # Create use case
│   │   ├── get_{entity}.rs       # Read use case
│   │   ├── update_{entity}.rs    # Update use case
│   │   └── delete_{entity}.rs    # Delete use case
│   │
│   ├── infrastructure/
│   │   ├── http/
│   │   │   ├── mod.rs            # Re-exports
│   │   │   ├── handlers.rs       # HTTP handlers
│   │   │   ├── routes.rs         # Route configuration
│   │   │   └── dto.rs            # Request/Response DTOs
│   │   │
│   │   └── persistence/
│   │       ├── mod.rs
│   │       └── postgres_{entity}_repository.rs
│   │
│   └── lib.rs                    # Module entry point
│
└── Cargo.toml
```

## Core Principles

### 1. Separation of Concerns

**Rule**: Each layer has a single, well-defined responsibility.

**Benefits**:
- Easier to test in isolation
- Can swap implementations without affecting other layers
- Clearer code organization

**Example**:
```rust
// ❌ BAD: Handler doing too much
pub async fn create_user(Json(req): Json<CreateUserRequest>) -> Result<impl IntoResponse> {
    // Validation in handler
    if req.email.is_empty() { return Err(...); }

    // Direct database access
    let pool = PgPool::connect("...").await?;
    sqlx::query!("INSERT INTO users...").execute(&pool).await?;

    // Business logic in handler
    send_welcome_email(&req.email).await?;
    Ok(Json(response))
}

// ✅ GOOD: Clear separation
pub async fn create_user(
    State(use_case): State<Arc<CreateUser>>,
    Validated(req): Validated<CreateUserRequest>,  // Validation at framework level
) -> Result<impl IntoResponse> {
    let user = req.into_domain();
    let created = use_case.execute(user).await?;   // Business logic in use case
    Ok((StatusCode::CREATED, Json(UserResponse::from(created))))
}
```

### 2. Dependency Inversion

**Rule**: High-level modules don't depend on low-level modules. Both depend on abstractions.

**Implementation**:
```rust
// Domain defines the contract
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User>;
}

// Application uses the trait
pub struct CreateUser {
    repository: Arc<dyn UserRepository>,  // Depends on abstraction
}

// Infrastructure implements the trait
pub struct PostgresUserRepository { ... }

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<User> { ... }
}
```

**Benefits**:
- Easy to swap PostgreSQL for MongoDB, Redis, etc.
- Can mock repositories for testing
- Business logic stays database-agnostic

### 3. Type Safety

**Rule**: Use Rust's type system to prevent bugs at compile time.

**Patterns**:

```rust
// ✅ Use enums for fixed sets of values
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
    Investment,
}

// ✅ Use NewTypes for domain concepts
pub struct Email(String);
pub struct PasswordHash(String);

// ✅ Use Option for nullable values
pub struct User {
    pub first_name: Option<String>,  // Clear: can be missing
    pub email: String,                // Clear: required
}

// ✅ Use Result for operations that can fail
pub async fn create_user(&self, user: User) -> Result<User>;
```

### 4. Immutability by Default

**Rule**: Data should be immutable unless mutation is required.

```rust
// ✅ Immutable by default
let user = User::new(email, password);

// ✅ Explicit mutability when needed
let mut total = 0;
for transaction in transactions {
    total += transaction.amount;
}

// ✅ Transform with methods that return new values
let updated_user = user.with_verified_email();
```

### 5. Error Handling

**Rule**: Use Result types and custom errors. Never panic in production code.

```rust
// ✅ Custom error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

// ✅ Return Result
pub async fn find_user(&self, id: Uuid) -> Result<User> {
    self.repository
        .find_by_id(id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("User {id} not found")))
}
```

## Design Patterns

### Repository Pattern

**Purpose**: Abstract data access logic from business logic.

**Implementation**:

```rust
// 1. Define trait in domain layer
#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Account>>;
    async fn create(&self, account: Account) -> Result<Account>;
    async fn update(&self, account: Account) -> Result<Account>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

// 2. Implement in infrastructure layer
pub struct PostgresAccountRepository {
    pool: Arc<PgPool>,
}

#[async_trait]
impl AccountRepository for PostgresAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>> {
        let account = sqlx::query_as!(
            Account,
            r#"
            SELECT id, user_id, name, account_type as "account_type: AccountType",
                   balance, currency, is_active, created_at, updated_at
            FROM accounts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;

        Ok(account)
    }
}

// 3. Use in application layer
pub struct GetAccount {
    repository: Arc<dyn AccountRepository>,
}

impl GetAccount {
    pub async fn execute(&self, id: Uuid) -> Result<Account> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Account {id} not found")))
    }
}
```

### Use Case Pattern

**Purpose**: Encapsulate single business operations with clear inputs and outputs.

**Rules**:
- One public method: `execute()`
- Single responsibility
- Stateless (all state in parameters)

**Example**:

```rust
pub struct CreateTransaction {
    repository: Arc<dyn TransactionRepository>,
    account_repository: Arc<dyn AccountRepository>,
}

impl CreateTransaction {
    pub async fn execute(&self, transaction: Transaction) -> Result<Transaction> {
        // 1. Validate
        let account = self.account_repository
            .find_by_id(transaction.account_id)
            .await?
            .ok_or_else(|| Error::NotFound("Account not found".to_string()))?;

        // 2. Business logic
        if !account.is_active {
            return Err(Error::Validation("Account is not active".to_string()));
        }

        // 3. Persist
        let created = self.repository.create(transaction).await?;

        Ok(created)
    }
}
```

### Service Layer Pattern

**Purpose**: Encapsulate external service integrations.

**Example**:

```rust
#[derive(Clone)]
pub struct EmailService {
    mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>,
    from_email: String,
    from_name: String,
}

impl EmailService {
    pub async fn send_verification_email(
        &self,
        to_email: &str,
        otp_code: &str,
    ) -> Result<()> {
        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse()?)
            .to(to_email.parse()?)
            .subject("Verify your email")
            .body(format!("Your verification code is: {}", otp_code))?;

        self.mailer.send(email).await?;
        Ok(())
    }
}
```

### Middleware Pattern

**Purpose**: Cross-cutting concerns like authentication, logging, error handling.

**Example**:

```rust
pub struct RequireAuth;

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        // Extract token from Authorization header
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| Error::Unauthorized("Missing token".to_string()))?;

        // Validate token (would use JWT service in real implementation)
        validate_token(token)?;

        Ok(RequireAuth)
    }
}

// Usage in handlers
pub async fn protected_endpoint(
    _auth: RequireAuth,  // Automatically validates authentication
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    // Handler only runs if authentication succeeds
    Ok(Json("Protected data"))
}
```

## Database Patterns

### Compile-Time SQL Verification

**Tool**: SQLx with compile-time checked queries

**Benefits**:
- SQL errors caught at compile time
- Type-safe query results
- No runtime query parsing overhead

**Example**:

```rust
// sqlx::query_as! macro verifies SQL at compile time
let user = sqlx::query_as!(
    User,
    r#"
    SELECT id, email, password_hash, first_name, last_name,
           is_active, email_verified, google_id, two_factor_enabled,
           two_factor_secret, created_at, updated_at
    FROM users
    WHERE email = $1
    "#,
    email
)
.fetch_optional(&self.pool)
.await?;
```

**Setup**:

```bash
# Set DATABASE_URL for compile-time verification
export DATABASE_URL="postgresql://user:pass@localhost/db"

# SQLx verifies queries during build
cargo build
```

### Custom Migration Tool

**Location**: `fta-migration/`

**Why Custom**:
- Full control over migration process
- Integration with application code
- Custom migration tracking

**Migration Format**:

```sql
-- migrations/20250101000001_create_users_table.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```

**Running Migrations**:

```bash
# Run all pending migrations
make m

# Or directly
cd fta-migration && cargo run --bin fta-migration -- run
```

### Database Transaction Handling

**Pattern**: Use repository methods for simple operations, explicit transactions for complex ones.

```rust
// Simple operation - repository handles connection
pub async fn create(&self, user: User) -> Result<User> {
    let user = sqlx::query_as!(...)
        .fetch_one(&self.pool)
        .await?;
    Ok(user)
}

// Complex operation - explicit transaction
pub async fn transfer_funds(
    &self,
    from_account: Uuid,
    to_account: Uuid,
    amount: i64,
) -> Result<()> {
    let mut tx = self.pool.begin().await?;

    // Debit from source
    sqlx::query!(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2",
        amount, from_account
    )
    .execute(&mut *tx)
    .await?;

    // Credit to destination
    sqlx::query!(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        amount, to_account
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
```

### Connection Pool Configuration

**Pattern**: Singleton pool shared across application.

```rust
// fta-database/src/lib.rs
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
        .context("Failed to create database pool")
}

// main.rs
let db_pool = create_pool(&database_url).await?;
let user_repository = Arc::new(PostgresUserRepository::new(db_pool.clone()));
```

## Validation Strategy

### Zod-rs Schema Validation

**Tool**: zod-rs for TypeScript-like schema validation

**Pattern**:

```rust
// 1. Define request DTO
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAccountRequest {
    pub name: String,
    pub account_type: AccountType,
    pub currency: String,
    pub initial_balance: i64,
}

// 2. Implement Validatable trait
impl Validatable for CreateAccountRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("name", string().min(1).max(100))
            .field("account_type", string()) // Enum validated by serde
            .field("currency", string().length(3)) // ISO 4217 codes
            .field("initial_balance", number().min(0))
    }
}

// 3. Use Validated extractor in handler
pub async fn create_account(
    State(use_case): State<Arc<CreateAccount>>,
    Validated(req): Validated<CreateAccountRequest>,  // Automatic validation
) -> Result<impl IntoResponse> {
    // req is guaranteed to be valid here
    let account = Account::new(req.name, req.account_type, req.currency, req.initial_balance);
    let created = use_case.execute(account).await?;
    Ok((StatusCode::CREATED, Json(AccountResponse::from(created))))
}
```

### Validation Rules

**String Validation**:
```rust
string()
    .min(3)              // Minimum length
    .max(255)            // Maximum length
    .email()             // Email format
    .regex(r"^\d{6}$")   // Custom regex (6 digits)
    .length(3)           // Exact length
    .optional()          // Field is optional
```

**Number Validation**:
```rust
number()
    .min(0)              // Minimum value
    .max(100)            // Maximum value
    .positive()          // Must be > 0
    .negative()          // Must be < 0
```

**Complex Validation**:
```rust
impl Validatable for CreateBudgetRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("name", string().min(1).max(100))
            .field("amount", number().positive())
            .field("period", string()) // enum: monthly, yearly
            .field("start_date", string()) // Date validation
            .field("end_date", string().optional())
            // Custom validation for date ranges in use case layer
    }
}
```

### Validation Error Responses

**Automatic Response**:
```json
{
  "error": "Validation failed",
  "details": {
    "email": "Invalid email format",
    "password": "Must be at least 8 characters"
  }
}
```

## Error Handling

### Centralized Error Types

**Location**: `fta-errors/src/lib.rs`

**Implementation**:

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    Validation(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    InternalServer(String),
    Database(sqlx::Error),
    External(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::Validation(msg) => write!(f, "Validation error: {}", msg),
            Error::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Error::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            Error::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Error::InternalServer(msg) => write!(f, "Internal server error: {}", msg),
            Error::Database(err) => write!(f, "Database error: {}", err),
            Error::External(msg) => write!(f, "External service error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, "Not Found", msg.clone()),
            Error::Validation(msg) => (StatusCode::BAD_REQUEST, "Validation Error", msg.clone()),
            Error::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "Unauthorized", msg.clone()),
            Error::Forbidden(msg) => (StatusCode::FORBIDDEN, "Forbidden", msg.clone()),
            Error::Conflict(msg) => (StatusCode::CONFLICT, "Conflict", msg.clone()),
            Error::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Database error occurred".to_string(),
            ),
            Error::InternalServer(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                msg.clone(),
            ),
            Error::External(msg) => (
                StatusCode::BAD_GATEWAY,
                "External Service Error",
                msg.clone(),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message,
        });

        (status, body).into_response()
    }
}

// Automatic conversions from common error types
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Database(err)
    }
}
```

### Error Handling in Layers

**Use Case Layer**:
```rust
pub async fn execute(&self, id: Uuid) -> Result<User> {
    // Return domain errors
    self.repository
        .find_by_id(id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("User {id} not found")))
}
```

**Handler Layer**:
```rust
pub async fn get_user(
    Path(id): Path<Uuid>,
    State(use_case): State<Arc<GetUser>>,
) -> Result<impl IntoResponse> {
    // Error automatically converted to HTTP response
    let user = use_case.execute(id).await?;
    Ok(Json(UserResponse::from(user)))
}
```

### Logging Errors

**Pattern**: Log at the point of handling, not at the point of creation.

```rust
// ✅ Log in middleware or at entry points
pub async fn error_handler(err: Error) -> Response {
    tracing::error!("Request failed: {:?}", err);
    err.into_response()
}

// ❌ Don't log in business logic
pub async fn execute(&self, id: Uuid) -> Result<User> {
    // Don't log here - let caller decide
    self.repository.find_by_id(id).await?
        .ok_or_else(|| Error::NotFound("User not found".to_string()))
}
```

## Authentication & Authorization

### JWT Dual-Token System

**Tokens**:
- **Access Token**: Short-lived (15 minutes), used for API requests
- **Refresh Token**: Long-lived (7 days), used to get new access tokens

**Implementation**:

```rust
pub struct JwtService {
    secret: String,
    access_token_expiry_minutes: i64,
    refresh_token_expiry_minutes: i64,
}

impl JwtService {
    pub fn new(
        secret: String,
        access_token_expiry_minutes: Option<i64>,
        refresh_token_expiry_minutes: Option<i64>,
    ) -> Self {
        Self {
            secret,
            access_token_expiry_minutes: access_token_expiry_minutes.unwrap_or(15),
            refresh_token_expiry_minutes: refresh_token_expiry_minutes.unwrap_or(10080), // 7 days
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(self.access_token_expiry_minutes))
            .unwrap()
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| Error::InternalServer(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<Uuid> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| Error::Unauthorized("Invalid token".to_string()))?;

        Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| Error::Unauthorized("Invalid user ID in token".to_string()))
    }
}
```

### Password Hashing

**Tool**: Argon2 (OWASP recommended)

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Clone)]
pub struct PasswordHashService {
    argon2: Argon2<'static>,
}

impl PasswordHashService {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(self
            .argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
```

### Two-Factor Authentication

**Protocol**: TOTP (Time-based One-Time Password)

```rust
pub struct TwoFactorService {
    issuer: String,
}

impl TwoFactorService {
    pub fn generate_secret(&self) -> String {
        let secret = totp_lite::totp_custom::<Sha1>(
            30,
            6,
            &[0u8; 20],
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        base32::encode(base32::Alphabet::RFC4648 { padding: false }, &secret.as_bytes())
    }

    pub fn verify_code(&self, secret: &str, code: &str) -> Result<bool> {
        let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret)
            .ok_or_else(|| Error::Validation("Invalid secret".to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expected = totp_lite::totp_custom::<Sha1>(30, 6, &secret_bytes, now);
        Ok(expected == code)
    }
}
```

### Authentication Middleware

**Pattern**: Axum extractors for authentication

```rust
pub struct RequireAuth {
    pub user_id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        // Extract JWT from Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| Error::Unauthorized("Missing Authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| Error::Unauthorized("Invalid Authorization format".to_string()))?;

        // Get JWT service from state
        let Extension(jwt_service) = Extension::<Arc<JwtService>>::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::InternalServer("JWT service not available".to_string()))?;

        // Verify token
        let user_id = jwt_service.verify_token(token)?;

        Ok(RequireAuth { user_id })
    }
}

// Usage
pub async fn protected_endpoint(
    auth: RequireAuth,  // Automatically validates and extracts user_id
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = auth.user_id;
    // Handler logic
    Ok(Json("Protected data"))
}
```

### OAuth Integration

**Provider**: Google OAuth 2.0

```rust
pub struct GoogleOAuthService {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl GoogleOAuthService {
    pub fn get_authorization_url(&self) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?\
             client_id={}&\
             redirect_uri={}&\
             response_type=code&\
             scope=openid%20email%20profile",
            self.client_id, self.redirect_uri
        )
    }

    pub async fn exchange_code_for_token(&self, code: &str) -> Result<GoogleTokenResponse> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("code", code),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("redirect_uri", &self.redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await?;

        Ok(response.json::<GoogleTokenResponse>().await?)
    }
}
```

## API Design

### RESTful Conventions

**Resource Naming**:
```
GET    /api/v1/users           # List users
GET    /api/v1/users/:id       # Get specific user
POST   /api/v1/users           # Create user
PUT    /api/v1/users/:id       # Update user (full)
PATCH  /api/v1/users/:id       # Update user (partial)
DELETE /api/v1/users/:id       # Delete user
```

**Nested Resources**:
```
GET    /api/v1/accounts/:id/transactions    # List transactions for account
POST   /api/v1/accounts/:id/transactions    # Create transaction for account
```

### OpenAPI Documentation

**Tool**: utoipa for automatic OpenAPI generation

**Implementation**:

```rust
// fta-server/src/api_doc.rs
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Financial Tracker API",
        version = "1.0.0",
        description = "Production-ready financial tracking REST API",
    ),
    paths(
        // User endpoints
        fta_users::infrastructure::http::handlers::create_user,
        fta_users::infrastructure::http::handlers::get_user,
        fta_users::infrastructure::http::handlers::list_users,
        fta_users::infrastructure::http::handlers::update_user,
        fta_users::infrastructure::http::handlers::delete_user,

        // Account endpoints
        fta_accounts::infrastructure::http::handlers::create_account,
        // ... more endpoints
    ),
    components(schemas(
        // Request/Response DTOs
        fta_users::infrastructure::http::dto::CreateUserRequest,
        fta_users::infrastructure::http::dto::UserResponse,
        // ... more schemas
    )),
    tags(
        (name = "Users", description = "User management endpoints"),
        (name = "Accounts", description = "Account management endpoints"),
        (name = "Transactions", description = "Transaction management endpoints"),
        (name = "Budgets", description = "Budget management endpoints"),
        (name = "Auth", description = "Authentication endpoints"),
    )
)]
pub struct ApiDoc;
```

**Handler Annotations**:

```rust
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already exists"),
    )
)]
pub async fn create_user(
    State(use_case): State<Arc<CreateUser>>,
    Validated(req): Validated<CreateUserRequest>,
) -> Result<impl IntoResponse> {
    // Handler implementation
}
```

**Access Documentation**:
```
http://localhost:3000/docs
```

### Response Patterns

**Success Responses**:
```rust
// Single resource
Ok(Json(UserResponse::from(user)))

// List of resources
Ok(Json(users.into_iter().map(UserResponse::from).collect::<Vec<_>>()))

// Created resource
Ok((StatusCode::CREATED, Json(AccountResponse::from(account))))

// No content (delete)
Ok(StatusCode::NO_CONTENT)
```

**Error Responses**:
```json
{
  "error": "Validation Error",
  "message": "Email must be a valid email address"
}
```

### Versioning

**Pattern**: URL versioning for clear API evolution

```rust
let api_v1 = Router::new()
    .merge(user_routes())
    .merge(account_routes())
    .merge(transaction_routes());

let app = Router::new()
    .nest("/api/v1", api_v1);
```

## Testing Strategy

### Test Organization

```
fta-users/
├── src/
│   └── ...
└── tests/
    ├── integration_tests.rs    # Full stack integration tests
    ├── repository_tests.rs     # Database integration tests
    └── use_case_tests.rs       # Business logic tests
```

### Unit Tests

**Location**: Same file as implementation, in `#[cfg(test)]` module

```rust
// fta-users/src/domain/user.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            is_active: true,
            email_verified: false,
            google_id: None,
            two_factor_enabled: false,
            two_factor_secret: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(user.email, "test@example.com");
        assert!(!user.email_verified);
    }
}
```

### Repository Tests

**Pattern**: Test against real database (test database)

```rust
// tests/repository_tests.rs
use fta_database::create_pool;
use fta_users::domain::{User, UserRepository};
use fta_users::infrastructure::persistence::PostgresUserRepository;

#[tokio::test]
async fn test_create_user() {
    // Setup test database
    let pool = create_pool(&test_database_url()).await.unwrap();
    let repo = PostgresUserRepository::new(pool);

    // Create test user
    let user = User::new("test@example.com", "password_hash");
    let created = repo.create(user).await.unwrap();

    // Verify
    assert_eq!(created.email, "test@example.com");

    // Cleanup
    repo.delete(created.id).await.unwrap();
}
```

### Use Case Tests

**Pattern**: Mock repositories for isolation

```rust
// tests/use_case_tests.rs
use fta_users::application::CreateUser;
use fta_users::domain::{User, UserRepository};
use std::sync::Arc;
use async_trait::async_trait;

struct MockUserRepository {
    users: Arc<Mutex<Vec<User>>>,
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, user: User) -> Result<User> {
        let mut users = self.users.lock().unwrap();
        users.push(user.clone());
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.iter().find(|u| u.email == email).cloned())
    }
}

#[tokio::test]
async fn test_create_user_use_case() {
    let repo = Arc::new(MockUserRepository {
        users: Arc::new(Mutex::new(vec![])),
    });
    let use_case = CreateUser::new(repo);

    let user = User::new("test@example.com", "hash");
    let result = use_case.execute(user).await.unwrap();

    assert_eq!(result.email, "test@example.com");
}
```

### Integration Tests

**Pattern**: Full HTTP request/response testing

```rust
// tests/integration_tests.rs
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_create_user_endpoint() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/users")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"email":"test@example.com","password":"password123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific package
cargo test -p fta-users

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests
```

## Development Workflow

### Local Development Setup

1. **Install Dependencies**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
# macOS
brew install postgresql@17

# Linux (Ubuntu/Debian)
sudo apt install postgresql-17

# Install cargo-make
cargo install cargo-make
```

2. **Database Setup**:
```bash
# Create database
createdb financial_tracker

# Copy environment template
cp .env.example .env

# Edit .env with your database credentials
vim .env

# Run migrations
make m
```

3. **Run Development Server**:
```bash
# Build and run
cargo run --bin fta-server

# Or with auto-reload (install cargo-watch first)
cargo install cargo-watch
cargo watch -x 'run --bin fta-server'
```

### Code Quality Tools

**Clippy** (Linting):
```bash
# Check for common mistakes
cargo clippy --workspace -- -D warnings

# With all features
cargo clippy --workspace --all-features
```

**Rustfmt** (Formatting):
```bash
# Check formatting
cargo fmt --all -- --check

# Auto-format
cargo fmt --all
```

**Audit** (Security):
```bash
# Install cargo-audit
cargo install cargo-audit

# Check for vulnerabilities
cargo audit
```

### Git Workflow

**Branch Naming**:
```
feature/user-authentication
bugfix/account-balance-calculation
hotfix/security-vulnerability
refactor/repository-pattern
```

**Commit Messages**:
```
feat: Add two-factor authentication
fix: Correct transaction balance calculation
refactor: Extract email service to separate module
docs: Update API documentation
test: Add integration tests for accounts
```

### Continuous Integration

**GitHub Actions** (`.github/workflows/ci.yml`):
```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:17
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v2

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run migrations
        run: |
          cargo install sqlx-cli
          sqlx migrate run

      - name: Run tests
        run: cargo test --workspace

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Run clippy
        run: cargo clippy --workspace -- -D warnings
```

## Common Commands

### Development

```bash
# Build workspace
cargo build --workspace

# Run server
cargo run --bin fta-server

# Run with release optimizations
cargo run --release --bin fta-server

# Check code (faster than build)
cargo check --workspace
```

### Database

```bash
# Run migrations
make m

# Create new migration
cd fta-migration/migrations
touch $(date +%Y%m%d%H%M%S)_migration_name.sql

# Reset database (drop and recreate)
make db-reset
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_create_user

# Run with output
cargo test -- --nocapture

# Run doctests
cargo test --doc
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace

# Fix lints automatically
cargo clippy --workspace --fix

# Security audit
cargo audit
```

### Dependencies

```bash
# Add dependency to workspace
# Edit Cargo.toml [workspace.dependencies]

# Add dependency to package
cd fta-users
cargo add serde

# Update dependencies
cargo update

# Check outdated dependencies
cargo install cargo-outdated
cargo outdated
```

### Documentation

```bash
# Generate and open docs
cargo doc --open

# Generate docs for all workspace members
cargo doc --workspace --no-deps --open
```

## Environment Configuration

### Required Variables

```bash
# Database
DATABASE_URL=postgresql://user:password@localhost:5432/financial_tracker

# JWT Authentication
JWT_SECRET=your-super-secret-jwt-key-change-in-production

# SMTP Email Configuration
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your-email@gmail.com
SMTP_PASS=your-app-specific-password
SMTP_FROM_EMAIL=noreply@yourdomain.com
SMTP_FROM_NAME=Financial Tracker

# Google OAuth
GOOGLE_CLIENT_ID=your-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/api/v1/auth/google/callback

# Application
BASE_URL=http://localhost:3000
APP_NAME=Financial Tracker

# Logging
LOG_FORMAT=pretty  # pretty, json, compact
RUST_LOG=info,sqlx=warn,tower_http=debug,axum=debug
LOG_TO_FILE=false  # true for production
```

### Environment-Specific Configs

**Development** (`.env.development`):
```bash
RUST_LOG=debug
LOG_FORMAT=pretty
LOG_TO_FILE=false
DATABASE_URL=postgresql://localhost/financial_tracker_dev
```

**Production** (`.env.production`):
```bash
RUST_LOG=info,sqlx=warn
LOG_FORMAT=json
LOG_TO_FILE=true
DATABASE_URL=postgresql://prod-server/financial_tracker
JWT_SECRET=<strong-random-secret>
```

## Best Practices Summary

### Architecture
✅ Use Clean Architecture with clear layer separation
✅ Domain layer has no external dependencies
✅ Application layer uses repository traits, not implementations
✅ Infrastructure layer implements all external integrations

### Code Organization
✅ One module per business domain (users, accounts, etc.)
✅ Each module follows domain/application/infrastructure structure
✅ Use workspace for code sharing across modules
✅ Keep files focused and under 500 lines

### Type Safety
✅ Use enums for fixed value sets
✅ Use NewTypes for domain concepts
✅ Use Option for nullable values
✅ Use Result for fallible operations
✅ Leverage compile-time checks (sqlx, validation)

### Error Handling
✅ Centralized error types with IntoResponse
✅ Use thiserror for error definitions
✅ Map errors at layer boundaries
✅ Log errors at entry points, not in business logic
✅ Never panic in production code

### Database
✅ Use compile-time verified queries (sqlx)
✅ Connection pool with proper configuration
✅ Transactions for multi-step operations
✅ Custom migration tool for control
✅ Repository pattern for abstraction

### Validation
✅ Schema-based validation with zod-rs
✅ Validate at framework boundary (extractors)
✅ Domain validation in use cases
✅ Type system for compile-time validation

### Security
✅ Argon2 for password hashing
✅ JWT with short-lived access tokens
✅ Refresh tokens stored in database
✅ TOTP for two-factor authentication
✅ OAuth 2.0 for third-party login
✅ Middleware for authentication/authorization

### Testing
✅ Unit tests for domain logic
✅ Integration tests for repositories
✅ Use case tests with mocks
✅ HTTP integration tests for endpoints
✅ Test database for repository tests

### API Design
✅ RESTful conventions
✅ OpenAPI documentation
✅ Consistent error responses
✅ URL versioning (/api/v1)
✅ Semantic HTTP status codes

### Performance
✅ Async/await throughout
✅ Connection pooling
✅ Compile-time optimization (no runtime query parsing)
✅ Efficient serialization (serde)
✅ Minimal allocations in hot paths

### Development
✅ cargo-make for task automation
✅ Clippy for linting
✅ rustfmt for formatting
✅ cargo-audit for security
✅ Comprehensive logging with tracing

---

## Contributing

When contributing to this project, please follow these guidelines:

1. **Follow the Clean Architecture**: Changes should respect layer boundaries
2. **Add Tests**: All new features need tests
3. **Update Documentation**: Keep README and API docs current
4. **Run Quality Checks**: Format, lint, and test before committing
5. **Write Good Commits**: Use conventional commit messages

## License

MIT License - See LICENSE file for details

---

**Built with ❤️ using Rust**

For questions or suggestions, please open an issue on GitHub.
