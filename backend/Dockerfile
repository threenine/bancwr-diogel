# Multi-stage Dockerfile for Bancwr Diogel (Bunker)
# Uses cargo-chef + sccache + mold for optimal build speed

# Stage 1: Chef - prepares the build recipe
FROM rust:1.94-slim-bookworm AS chef
WORKDIR /usr/src/bunker
RUN cargo install cargo-chef sccache

# Stage 2: Planner - generates the recipe.json
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder - compiles dependencies and the app
FROM chef AS builder

# Install build dependencies + mold linker for faster linking
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    g++ \
    mold \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Enable sccache for faster compilation
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache
ENV SCCACHE_CACHE_SIZE=2G

# Use mold linker for faster linking
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold -D warnings"

# Copy the recipe and build dependencies (this layer is cached!)
COPY --from=planner /usr/src/bunker/recipe.json recipe.json
RUN --mount=type=cache,target=/sccache \
    cargo chef cook --release --recipe-path recipe.json

# Copy source and build the app
COPY . .
RUN --mount=type=cache,target=/sccache \
    cargo build --release

# Stage 4: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary
COPY --from=builder /usr/src/bunker/target/release/bunker /usr/local/bin/bunker

# Default port configuration
ARG DEFAULT_PORT=3000
ENV BUNKER_PORT=${DEFAULT_PORT}

EXPOSE ${DEFAULT_PORT}

# Healthcheck
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:${BUNKER_PORT}/health || exit 1

CMD ["bunker"]