# --- Step 1: Base image with cargo-chef ---
FROM rust:1.91-slim AS chef
# We install cargo-chef globally here so we can reuse this stage
RUN cargo install cargo-chef
WORKDIR /app

# --- Step 2: The Planner ---
FROM chef AS planner
COPY . .
# This step analyzes your Cargo.toml/lock and computes a dependency recipe
RUN cargo chef prepare --recipe-path recipe.json

# --- Step 3: The Builder ---
FROM chef AS builder
# Install system dependencies required for compiling C-bindings (like Diesel/postgres)
RUN apt-get update && apt-get install -y libpq-dev pkg-config libssl-dev curl

# Copy ONLY the recipe from the planner
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - THIS IS THE MASSIVE CACHE LAYER
# If your Cargo.toml hasn't changed, Docker will use the cached result of this command instantly!
RUN cargo chef cook --release --recipe-path recipe.json

# Now copy in your actual application source code
COPY . .
# Build the application itself. Because dependencies are already compiled, this is very fast.
RUN cargo build --release --workspace

# --- Step 4: The Runtime ---
FROM debian:trixie-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends \
  libpq5 \
  ca-certificates \
  libssl3 \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy binaries from the builder stage
COPY --from=builder /app/target/release/actix-diesel-async-starter /app/server
COPY --from=builder /app/target/release/migrator /app/migrator

# Copy configuration files
COPY config /app/config

# Set environment
ENV RUN_MODE=production

# Create non-root user for security
RUN groupadd -r appuser && useradd -r -g appuser appuser \
  && chown -R appuser:appuser /app
USER appuser

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["sh", "-c", "./migrator && exec ./server"]
