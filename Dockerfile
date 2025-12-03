FROM rust:1.83-slim-bookworm as builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/Cargo.toml
COPY domain/Cargo.toml domain/Cargo.toml
COPY infrastructure/Cargo.toml infrastructure/Cargo.toml

# Create dummy source files to cache dependencies
RUN mkdir -p api/src domain/src infrastructure/src
RUN echo "fn main() {}" > api/src/main.rs
RUN echo "" > domain/src/lib.rs
RUN echo "" > infrastructure/src/lib.rs

# Build dependencies
RUN cargo build --release

# Copy actual source code
COPY . .

# Touch main.rs to force rebuild
RUN touch api/src/main.rs

# Build application
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /app/server

EXPOSE 8080

CMD ["./server"]
