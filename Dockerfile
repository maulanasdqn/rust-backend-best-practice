FROM rust:1.85-alpine AS builder

RUN apk update && apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    curl \
    pkgconfig \
    libc6-compat

RUN cargo install sqlx-cli --no-default-features --features postgres,native-tls

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY fta-accounts/Cargo.toml ./fta-accounts/
COPY fta-auth/Cargo.toml ./fta-auth/
COPY fta-budgets/Cargo.toml ./fta-budgets/
COPY fta-database/Cargo.toml ./fta-database/
COPY fta-errors/Cargo.toml ./fta-errors/
COPY fta-migration/Cargo.toml ./fta-migration/
COPY fta-server/Cargo.toml ./fta-server/
COPY fta-transactions/Cargo.toml ./fta-transactions/
COPY fta-types/Cargo.toml ./fta-types/
COPY fta-users/Cargo.toml ./fta-users/
COPY fta-validation/Cargo.toml ./fta-validation/

COPY . .

ENV OPENSSL_STATIC=true
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --bin fta-server --target x86_64-unknown-linux-musl
RUN cargo build --release --bin fta-migration --target x86_64-unknown-linux-musl

FROM alpine:3.19

RUN apk update && apk add --no-cache \
    ca-certificates \
    postgresql-client \
    libc6-compat

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fta-server /fta-server
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fta-migration /fta-migration
COPY --from=builder /usr/local/cargo/bin/sqlx /sqlx
COPY --from=builder /app/fta-migration/migrations /migrations

COPY docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

EXPOSE 3000

ENTRYPOINT ["/docker-entrypoint.sh"]
