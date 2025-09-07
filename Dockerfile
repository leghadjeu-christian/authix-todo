# Stage 1: Builder
FROM rust:latest AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y openssl libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Copy manifests first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Warm up dependencies with dummy main.rs
RUN mkdir src \
    && echo "fn main() { println!(\"hello placeholder\"); }" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Copy real source code and migrations
COPY src ./src
COPY javascript ./javascript
COPY css ./css
COPY templates ./templates
COPY migrations ./migrations

# Clean old placeholder build and rebuild actual binary
RUN cargo clean && cargo build --release

# Install diesel_cli into /usr/local/bin
RUN cargo install diesel_cli --no-default-features --features postgres --root /usr/local

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime deps
RUN apt-get update && apt-get install -y ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy Actix app binary
COPY --from=builder /app/target/release/web_application ./web_application

# Copy diesel_cli for migrations
COPY --from=builder /usr/local/bin/diesel /usr/local/bin/diesel

# Copy static assets and migrations
COPY --from=builder /app/javascript ./javascript
COPY --from=builder /app/css ./css
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/migrations ./migrations

EXPOSE 8000

CMD ["./web_application"]
