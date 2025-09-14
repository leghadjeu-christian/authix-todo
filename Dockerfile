# Stage 1: Builder
FROM rust:1.82.0-slim-bullseye AS builder
WORKDIR /app

# Install system dependencies for Diesel
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Warm up dependencies with a dummy main.rs (to cache deps)
RUN mkdir src \
    && echo "fn main() { println!(\"hello placeholder\"); }" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Install diesel_cli (same glibc as runtime)
RUN cargo install diesel_cli --version 2.2.12 --no-default-features --features postgres --root /usr/local

# Copy real source code and migrations
COPY src ./src
COPY javascript ./javascript
COPY css ./css
COPY templates ./templates
COPY migrations ./migrations
COPY diesel.toml ./diesel.toml

# Build the real application
RUN cargo clean && cargo build --release --bin web_application

# Stage 2: Runtime
FROM debian:bullseye-slim
WORKDIR /app

# Install only required runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled Actix app binary
COPY --from=builder /app/target/release/web_application ./web_application

# Copy diesel_cli into runtime (for migrations in initContainer)
COPY --from=builder /usr/local/bin/diesel /usr/local/bin/diesel

# Copy static assets and migrations
COPY --from=builder /app/javascript ./javascript
COPY --from=builder /app/css ./css
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/diesel.toml ./diesel.toml

EXPOSE 8000

CMD ["./web_application"]
