# Axum Clean Architecture

> A production-ready Rust backend boilerplate using Clean Architecture, Domain-Driven Design, and modern Rust patterns.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/axum-0.8-blue.svg)](https://github.com/tokio-rs/axum)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Clean Architecture** with clear separation of concerns
- **Domain-Driven Design** principles
- **Modular Monolith** structure (easily split into microservices)
- **JWT Authentication** with access/refresh token pattern
- **Two-Factor Authentication** (TOTP)
- **Google OAuth 2.0** integration
- **OpenAPI/Swagger** documentation
- **Rate Limiting** and request body size limits
- **Structured Logging** with tracing
- **Type-safe SQL** with SeaORM
- **Comprehensive Error Handling**
- **Request Validation** with zod-rs

## Quick Start

### Prerequisites

- Rust 1.75+ ([install](https://rustup.rs))
- PostgreSQL 15+ ([install](https://www.postgresql.org/download/))
- Docker & Docker Compose (optional, for containerized setup)

### Option 1: Local Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/axum-clean-architecture.git
cd axum-clean-architecture

# Copy environment template
cp .env.example .env

# Edit .env with your configuration
vim .env

# Create database
createdb your_database_name

# Run migrations
cd fta-migration && cargo run

# Start the server
cargo run --bin fta-server
```

### Option 2: Docker Setup

```bash
# Clone and setup
git clone https://github.com/yourusername/axum-clean-architecture.git
cd axum-clean-architecture
cp .env.example .env

# Start all services
docker-compose up -d

# Run migrations
docker-compose exec app cargo run -p fta-migration

# Server is available at http://localhost:3000
```

### Verify Installation

```bash
# Health check
curl http://localhost:3000/health

# API Documentation
open http://localhost:3000/docs
```

## Project Structure

```
axum-clean-architecture/
├── fta-server/           # Application entry point
├── fta-auth/             # Authentication module
├── fta-users/            # User management module
├── fta-accounts/         # Account management (example domain)
├── fta-transactions/     # Transaction management (example domain)
├── fta-budgets/          # Budget management (example domain)
├── fta-database/         # Database connection pool
├── fta-errors/           # Centralized error types
├── fta-types/            # Shared types and utilities
├── fta-validation/       # Validation framework
├── fta-migration/        # Database migrations
└── fta-test-utils/       # Testing utilities
```

### Module Architecture

Each domain module follows Clean Architecture:

```
fta-{module}/
├── src/
│   ├── domain/           # Entities, repository traits
│   ├── application/      # Use cases (business logic)
│   └── infrastructure/
│       ├── http/         # Handlers, routes, DTOs
│       └── persistence/  # Repository implementations
└── Cargo.toml
```

## Architecture Overview

```
┌────────────────────────────────────────────────────────────┐
│                    Infrastructure Layer                     │
│  (HTTP Handlers, Repositories, External Services, DTOs)     │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │               Application Layer                      │   │
│  │         (Use Cases, Business Logic)                  │   │
│  │                                                      │   │
│  │  ┌───────────────────────────────────────────────┐  │   │
│  │  │              Domain Layer                      │  │   │
│  │  │   (Entities, Value Objects, Domain Traits)     │  │   │
│  │  └───────────────────────────────────────────────┘  │   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└────────────────────────────────────────────────────────────┘
```

### Dependency Flow

```
HTTP Request → Handler → Use Case → Repository Trait
                                          ↑
                              Repository Implementation → Database
```

## Technology Stack

| Category | Technology |
|----------|------------|
| Framework | Axum 0.8 |
| Runtime | Tokio |
| Database | PostgreSQL + SeaORM |
| Auth | JWT + Argon2 + TOTP |
| Validation | zod-rs |
| Documentation | utoipa (OpenAPI 3.0) |
| Logging | tracing |
| Testing | tokio-test |

## API Endpoints

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/auth/register` | Register new user |
| POST | `/api/v1/auth/login` | Login |
| POST | `/api/v1/auth/logout` | Logout |
| POST | `/api/v1/auth/refresh` | Refresh access token |
| POST | `/api/v1/auth/change-password` | Change password |
| GET | `/api/v1/auth/google` | Google OAuth login |
| POST | `/api/v1/auth/2fa/enable` | Enable 2FA |
| POST | `/api/v1/auth/2fa/verify` | Verify 2FA code |

### Users
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/users` | List users |
| GET | `/api/v1/users/:id` | Get user |
| POST | `/api/v1/users` | Create user |
| PATCH | `/api/v1/users/:id` | Update user |
| DELETE | `/api/v1/users/:id` | Delete user |

### Example Domains (Accounts, Transactions, Budgets)

Similar CRUD endpoints available for each domain module.

Full API documentation available at `/docs` when server is running.

## Configuration

See [.env.example](.env.example) for all available configuration options.

### Required Environment Variables

```bash
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
JWT_SECRET=your-secret-key
```

### Optional Variables

```bash
# Server
HOST=0.0.0.0
PORT=3000
BASE_URL=http://localhost:3000

# Email (SMTP)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your-email@gmail.com
SMTP_PASS=your-app-password

# OAuth
GOOGLE_CLIENT_ID=your-client-id
GOOGLE_CLIENT_SECRET=your-client-secret

# Logging
LOG_FORMAT=pretty  # pretty, json, compact
RUST_LOG=info
```

## Development

### Commands

```bash
# Build
cargo build --workspace

# Run development server
cargo run --bin fta-server

# Run with auto-reload
cargo watch -x 'run --bin fta-server'

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace

# Generate documentation
cargo doc --workspace --open
```

### Adding a New Domain Module

1. Create new crate:
```bash
cargo new fta-your-module --lib
```

2. Follow the module structure:
```
fta-your-module/
├── src/
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── entity.rs
│   │   └── repository.rs
│   ├── application/
│   │   ├── mod.rs
│   │   └── use_cases.rs
│   ├── infrastructure/
│   │   ├── http/
│   │   └── persistence/
│   └── lib.rs
└── Cargo.toml
```

3. Add to workspace in root `Cargo.toml`

4. Wire up routes in `fta-server/src/router.rs`

## Security Features

- **Password Hashing**: Argon2 (OWASP recommended)
- **JWT Tokens**: Short-lived access tokens (15 min) + refresh tokens (7 days)
- **Rate Limiting**: 50 requests/second per IP (configurable)
- **Body Size Limit**: 1MB max request body
- **2FA Support**: TOTP-based two-factor authentication
- **OAuth 2.0**: Google OAuth integration

## Testing

```bash
# Run all tests
cargo test --workspace

# Run specific module tests
cargo test -p fta-users

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

## Deployment

### Docker

```bash
# Build image
docker build -t axum-clean-architecture .

# Run container
docker run -p 3000:3000 --env-file .env axum-clean-architecture
```

### Production Checklist

- [ ] Set strong `JWT_SECRET`
- [ ] Configure production `DATABASE_URL`
- [ ] Set `RUST_LOG=info,sqlx=warn`
- [ ] Set `LOG_FORMAT=json`
- [ ] Enable HTTPS (via reverse proxy)
- [ ] Configure CORS for your domains
- [ ] Set up monitoring and alerting

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SeaORM](https://www.sea-ql.org/SeaORM/) - ORM
- [utoipa](https://github.com/juhaku/utoipa) - OpenAPI documentation

---

**Built with Rust**
