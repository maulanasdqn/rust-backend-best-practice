# Build stage
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY abbp-server/Cargo.toml abbp-server/
COPY abbp-auth/Cargo.toml abbp-auth/
COPY abbp-users/Cargo.toml abbp-users/
COPY abbp-accounts/Cargo.toml abbp-accounts/
COPY abbp-transactions/Cargo.toml abbp-transactions/
COPY abbp-budgets/Cargo.toml abbp-budgets/
COPY abbp-database/Cargo.toml abbp-database/
COPY abbp-errors/Cargo.toml abbp-errors/
COPY abbp-types/Cargo.toml abbp-types/
COPY abbp-validation/Cargo.toml abbp-validation/
COPY abbp-migration/Cargo.toml abbp-migration/
COPY abbp-test-utils/Cargo.toml abbp-test-utils/

# Create dummy source files for dependency caching
RUN mkdir -p abbp-server/src && echo "fn main() {}" > abbp-server/src/main.rs
RUN for dir in abbp-auth abbp-users abbp-accounts abbp-transactions abbp-budgets abbp-database abbp-errors abbp-types abbp-validation abbp-migration abbp-test-utils; do \
      mkdir -p $dir/src && echo "pub fn dummy() {}" > $dir/src/lib.rs; \
    done

# Build dependencies only
RUN cargo build --release --bin abbp-server 2>/dev/null || true

# Copy actual source code
COPY . .

# Touch source files to rebuild with actual code
RUN find . -name "*.rs" -exec touch {} \;

# Build the application
RUN cargo build --release --bin abbp-server

# Runtime stage
FROM alpine:3.19

RUN apk add --no-cache ca-certificates libgcc

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/abbp-server /app/abbp-server

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

EXPOSE 3000

ENV RUST_LOG=info

CMD ["./abbp-server"]
