# Contributing to Axum Backend Best Practice

First off, thank you for considering contributing to this project! It's people like you that make this boilerplate better for everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Architecture Guidelines](#architecture-guidelines)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)

## Code of Conduct

This project and everyone participating in it is governed by our commitment to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/axum-backend-best-practice.git
   cd axum-backend-best-practice
   ```
3. **Set up the development environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your local settings
   docker-compose up -d postgres
   cd abbp-migration && cargo run
   ```
4. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Process

### Branch Naming Convention

- `feature/` - New features (e.g., `feature/add-oauth-github`)
- `fix/` - Bug fixes (e.g., `fix/user-validation-error`)
- `refactor/` - Code refactoring (e.g., `refactor/repository-pattern`)
- `docs/` - Documentation changes (e.g., `docs/update-readme`)
- `test/` - Test additions/changes (e.g., `test/user-integration`)

### Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat` - A new feature
- `fix` - A bug fix
- `docs` - Documentation only changes
- `style` - Changes that don't affect code meaning (formatting, etc.)
- `refactor` - Code change that neither fixes a bug nor adds a feature
- `test` - Adding or modifying tests
- `chore` - Changes to build process or auxiliary tools

**Examples:**
```
feat(auth): add GitHub OAuth integration
fix(users): correct email validation regex
docs(readme): add deployment instructions
refactor(accounts): extract balance calculation to domain
test(transactions): add integration tests for transfers
```

## Pull Request Process

1. **Ensure your code compiles** without warnings:
   ```bash
   cargo build --workspace
   cargo clippy --workspace -- -D warnings
   ```

2. **Format your code**:
   ```bash
   cargo fmt --all
   ```

3. **Run all tests**:
   ```bash
   cargo test --workspace
   ```

4. **Update documentation** if you've changed APIs or added features

5. **Create the Pull Request** with:
   - Clear title following commit message format
   - Description of what changes were made and why
   - Link to any related issues
   - Screenshots if UI/API changes are involved

6. **Address review feedback** promptly and push additional commits

7. **Squash commits** if requested before merge

## Coding Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting (already configured in `rustfmt.toml`)
- Fix all `clippy` warnings before submitting

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Modules | snake_case | `user_repository` |
| Types/Structs | PascalCase | `UserRepository` |
| Functions | snake_case | `find_by_id` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS` |
| Type Parameters | Single uppercase | `T`, `E` |

### Error Handling

- Use `AppError` for all application errors
- Provide meaningful error messages
- Never use `.unwrap()` in production code (use `.expect()` with context or `?`)
- Log errors at appropriate levels

### Documentation

- Add rustdoc comments (`///`) for public APIs
- Include examples in doc comments where helpful
- Document error conditions in `# Errors` section

## Architecture Guidelines

### Clean Architecture Principles

1. **Domain Layer** (innermost)
   - Pure business logic
   - No external dependencies (only std, serde, chrono, uuid)
   - Entities and repository traits

2. **Application Layer** (middle)
   - Use cases with single responsibility
   - Depends only on domain layer
   - No framework-specific code

3. **Infrastructure Layer** (outermost)
   - HTTP handlers, database implementations
   - External service integrations
   - Framework-specific code

### Adding a New Domain Module

1. Create the crate structure:
   ```
   abbp-your-module/
   ├── src/
   │   ├── domain/
   │   │   ├── mod.rs
   │   │   ├── entity.rs
   │   │   └── repository.rs
   │   ├── application/
   │   │   ├── mod.rs
   │   │   ├── create_entity.rs
   │   │   ├── get_entity.rs
   │   │   └── ...
   │   ├── infrastructure/
   │   │   ├── http/
   │   │   │   ├── mod.rs
   │   │   │   ├── handlers.rs
   │   │   │   ├── routes.rs
   │   │   │   └── dto.rs
   │   │   └── persistence/
   │   │       ├── mod.rs
   │   │       └── postgres_repository.rs
   │   └── lib.rs
   └── Cargo.toml
   ```

2. Add to workspace in root `Cargo.toml`

3. Create migrations in `abbp-migration/migrations/`

4. Wire up routes in `abbp-server/src/router.rs`

5. Add OpenAPI documentation

## Testing Guidelines

### Test Organization

- **Unit tests**: In the same file as the code, in `#[cfg(test)]` module
- **Integration tests**: In `tests/` directory of each crate
- **End-to-end tests**: In `abbp-test-utils` or separate test crate

### Test Naming

```rust
#[test]
fn test_<function_name>_<scenario>_<expected_result>() {
    // ...
}

// Examples:
fn test_create_user_with_valid_data_succeeds() { }
fn test_create_user_with_duplicate_email_returns_conflict() { }
fn test_login_with_wrong_password_returns_unauthorized() { }
```

### Test Coverage

- All public APIs should have tests
- Edge cases and error conditions should be tested
- Integration tests for database operations
- Use mocks for external services

## Documentation

### When to Document

- All public modules, structs, traits, and functions
- Complex algorithms or business logic
- Configuration options
- API endpoints (via OpenAPI annotations)

### Documentation Format

```rust
/// Brief description of what this does.
///
/// More detailed explanation if needed, including:
/// - Important behaviors
/// - Side effects
/// - Related functions
///
/// # Arguments
///
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
///
/// Description of what is returned
///
/// # Errors
///
/// * `AppError::NotFound` - When the resource doesn't exist
/// * `AppError::Unauthorized` - When authentication fails
///
/// # Examples
///
/// ```rust
/// let result = function_name(arg1, arg2)?;
/// assert!(result.is_ok());
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType, AppError> {
    // ...
}
```

---

## Questions?

If you have questions about contributing, feel free to:
- Open a GitHub issue with the `question` label
- Start a discussion in the Discussions tab

Thank you for contributing!
