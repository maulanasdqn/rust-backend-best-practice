# Build stage
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY fta-server/Cargo.toml fta-server/
COPY fta-auth/Cargo.toml fta-auth/
COPY fta-users/Cargo.toml fta-users/
COPY fta-accounts/Cargo.toml fta-accounts/
COPY fta-transactions/Cargo.toml fta-transactions/
COPY fta-budgets/Cargo.toml fta-budgets/
COPY fta-database/Cargo.toml fta-database/
COPY fta-errors/Cargo.toml fta-errors/
COPY fta-types/Cargo.toml fta-types/
COPY fta-validation/Cargo.toml fta-validation/
COPY fta-migration/Cargo.toml fta-migration/
COPY fta-test-utils/Cargo.toml fta-test-utils/

# Create dummy source files for dependency caching
RUN mkdir -p fta-server/src && echo "fn main() {}" > fta-server/src/main.rs
RUN for dir in fta-auth fta-users fta-accounts fta-transactions fta-budgets fta-database fta-errors fta-types fta-validation fta-migration fta-test-utils; do \
      mkdir -p $dir/src && echo "pub fn dummy() {}" > $dir/src/lib.rs; \
    done

# Build dependencies only
RUN cargo build --release --bin fta-server 2>/dev/null || true

# Copy actual source code
COPY . .

# Touch source files to rebuild with actual code
RUN find . -name "*.rs" -exec touch {} \;

# Build the application
RUN cargo build --release --bin fta-server

# Runtime stage
FROM alpine:3.19

RUN apk add --no-cache ca-certificates libgcc

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/fta-server /app/fta-server

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

EXPOSE 3000

ENV RUST_LOG=info

CMD ["./fta-server"]
