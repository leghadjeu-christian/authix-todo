# Stage 1: Builder
FROM rust:latest AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y openssl libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Copy manifests first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Warm up dependencies with a dummy main.rs (to cache deps)
RUN mkdir src \
    && echo "fn main() { println!(\"hello placeholder\"); }" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Install diesel_cli into /usr/local/bin (instead of cargo home)
RUN cargo install diesel_cli --no-default-features --features postgres --root /usr/local

# Copy real source code and migrations
COPY src ./src
COPY javascript ./javascript
COPY css ./css
COPY templates ./templates
COPY migrations ./migrations

# Force rebuild the actual application (remove placeholder build artifacts)
RUN cargo clean && cargo build --release --bin web_application

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install only required runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy the compiled Actix app binary
COPY --from=builder /app/target/release/web_application ./web_application

# Copy diesel_cli into runtime (for migrations in initContainer)
COPY --from=builder /usr/local/bin/diesel /usr/local/bin/diesel

# Copy static assets and migrations
COPY --from=builder /app/javascript ./javascript
COPY --from=builder /app/css ./css
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/migrations ./migrations

# Expose app port
EXPOSE 8000

# Run Actix app
CMD ["./web_application"]
