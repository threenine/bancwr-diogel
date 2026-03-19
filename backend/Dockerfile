# Multi-stage Dockerfile for Bancwr Diogel (Bunker)

# Stage 1: Build
FROM rust:1.81-slim-bookworm as builder

WORKDIR /usr/src/bunker

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy source for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "pub mod config; pub mod signer; pub mod server;" > src/lib.rs

# Build dependencies only (caching)
RUN cargo build --release
RUN rm -f target/release/deps/bunker*

# Copy actual source code
COPY src/ ./src/

# Final build
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL is required by nostr-sdk/reqwest)
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/bunker/target/release/bunker /usr/local/bin/bunker

# Default port configuration
ARG DEFAULT_PORT=3000
ENV BUNKER_PORT=${DEFAULT_PORT}

# Expose the port
EXPOSE ${DEFAULT_PORT}

# Healthcheck
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:${BUNKER_PORT}/health || exit 1

# Run the binary
CMD ["bunker"]
