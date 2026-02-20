# Axum Backend Best Practice - Testing Makefile
# Better UI for running tests

.PHONY: help test test-watch test-fast test-coverage test-coverage-open clean lint lint-fix lint-strict lint-watch lint-package format format-check migrate migrate-revert migrate-info migrate-add migrate-reset db-create db-drop server server-watch server-release t tw tc l lf ls f fc m mr mi s sw sr

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

test: ## Run all tests with nextest (better UI)
	cargo nextest run

test-watch: ## Watch and re-run tests on file changes
	cargo watch -x "nextest run"

test-fast: ## Run tests in fast mode (more parallel)
	cargo nextest run --profile fast

test-coverage: ## Generate HTML coverage report
	cargo llvm-cov nextest --html
	@echo "Coverage report generated at: target/llvm-cov/html/index.html"

test-coverage-open: ## Generate and open coverage report
	cargo llvm-cov nextest --open

test-coverage-lcov: ## Generate coverage in lcov format
	cargo llvm-cov nextest --lcov --output-path lcov.info

test-package: ## Test specific package (usage: make test-package PKG=abbp-types)
	cargo nextest run -p $(PKG)

test-unit: ## Run only unit tests
	cargo nextest run --lib

test-ci: ## Run tests in CI mode with retries
	cargo nextest run --profile ci

test-verbose: ## Run tests with verbose output
	cargo nextest run --success-output immediate

test-clean: ## Clean test artifacts
	cargo clean

# Quick shortcuts
t: test ## Shortcut for 'test'
tw: test-watch ## Shortcut for 'test-watch'
tc: test-coverage-open ## Shortcut for 'test-coverage-open'

# Code Quality - Linting
lint: ## Run clippy linter on all workspace crates
	cargo clippy --workspace --all-targets

lint-fix: ## Run clippy with automatic fixes
	cargo clippy --workspace --all-targets --fix --allow-dirty --allow-staged

lint-strict: ## Run clippy with warnings as errors (CI mode)
	cargo clippy --workspace --all-targets -- -D warnings

lint-watch: ## Watch and re-run clippy on file changes
	cargo watch -x "clippy --workspace --all-targets"

lint-package: ## Lint specific package (usage: make lint-package PKG=abbp-types)
	cargo clippy -p $(PKG) --all-targets

# Code Quality - Formatting
format: ## Format all code with rustfmt
	cargo fmt --all

format-check: ## Check code formatting without modifying files
	cargo fmt --all -- --check

# Quality shortcuts
l: lint ## Shortcut for 'lint'
lf: lint-fix ## Shortcut for 'lint-fix'
ls: lint-strict ## Shortcut for 'lint-strict'
f: format ## Shortcut for 'format'
fc: format-check ## Shortcut for 'format-check'

# Database Migration Commands
migrate: ## Run all pending migrations
	cd abbp-migration && cargo run --bin abbp-migration -- run

migrate-revert: ## Revert the last migration
	cd abbp-migration && cargo run --bin abbp-migration -- revert

migrate-info: ## Show migration status
	cd abbp-migration && cargo run --bin abbp-migration -- info

migrate-add: ## Create new migration (usage: make migrate-add NAME=create_foo)
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME is required. Usage: make migrate-add NAME=create_foo"; \
		exit 1; \
	fi
	cd abbp-migration && cargo run --bin abbp-migration -- add "$(NAME)"

migrate-reset: ## Drop all tables and re-run migrations (DESTRUCTIVE!)
	@echo "⚠️  WARNING: This will DROP ALL TABLES!"
	@read -p "Are you sure? Type 'yes' to continue: " confirm && [ "$$confirm" = "yes" ] || (echo "Aborted." && exit 1)
	sqlx database drop -y
	sqlx database create
	cd abbp-migration && cargo run --bin abbp-migration -- run

# Database Management
db-create: ## Create the database
	sqlx database create

db-drop: ## Drop the database (DESTRUCTIVE!)
	@echo "⚠️  WARNING: This will DROP THE DATABASE!"
	@read -p "Are you sure? Type 'yes' to continue: " confirm && [ "$$confirm" = "yes" ] || (echo "Aborted." && exit 1)
	sqlx database drop -y

db-reset: migrate-reset ## Alias for migrate-reset

# Migration shortcuts
m: migrate ## Shortcut for 'migrate'
mr: migrate-revert ## Shortcut for 'migrate-revert'
mi: migrate-info ## Shortcut for 'migrate-info'

# Development Server
server: ## Run the abbp-server in development mode
	cargo run -p abbp-server

server-watch: ## Run server with auto-reload on file changes
	cargo watch -x "run -p abbp-server"

server-release: ## Run server in release mode (optimized)
	cargo run -p abbp-server --release

# Server shortcuts
s: server ## Shortcut for 'server'
sw: server-watch ## Shortcut for 'server-watch'
sr: server-release ## Shortcut for 'server-release'

clean: ## Clean all build artifacts
	cargo clean
	rm -rf target/
	rm -rf lcov.info
