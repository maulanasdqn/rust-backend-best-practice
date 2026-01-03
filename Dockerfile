FROM rust:1-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY fta-accounts ./fta-accounts
COPY fta-auth ./fta-auth
COPY fta-budgets ./fta-budgets
COPY fta-database ./fta-database
COPY fta-errors ./fta-errors
COPY fta-migration ./fta-migration
COPY fta-server ./fta-server
COPY fta-test-utils ./fta-test-utils
COPY fta-transactions ./fta-transactions
COPY fta-types ./fta-types
COPY fta-users ./fta-users
COPY fta-validation ./fta-validation

RUN cargo build --release -p fta-server
RUN cargo build --release -p fta-migration

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1001 -s /bin/bash appuser

WORKDIR /app

COPY --from=builder /app/target/release/fta-server /usr/local/bin/fta-server
COPY --from=builder /app/target/release/fta-migration /usr/local/bin/fta-migration

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 3000

CMD ["fta-server"]
