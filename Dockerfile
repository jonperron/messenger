#---------------------------------------------------------------------------------------
# Stage 1 → Builder image
#---------------------------------------------------------------------------------------
FROM rust:1.83-slim-bookworm as builder

# Install dependencies for the build
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

#---------------------------------------------------------------------------------------
# Stage 2 → Production image
#---------------------------------------------------------------------------------------
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
WORKDIR /app
COPY --from=builder /app/target/release/messenger .

# Copy templates and config
COPY templates ./templates/
COPY config.yaml ./config.yaml

# Set entrypoint
EXPOSE 3000
ENTRYPOINT ["./messenger"]