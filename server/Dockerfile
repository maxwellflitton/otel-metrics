# Use Rust official slim image
FROM rust:1.74-slim AS builder

# Set working directory
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files first to leverage Docker layer caching
COPY Cargo.toml Cargo.toml
COPY src src

# Build the Rust application
RUN cargo build --release
RUN cp /app/target/release/server /app/server

# # Final lightweight runtime image
# FROM debian:bullseye-slim

# # Install required system libraries for Rust & OpenTelemetry
# RUN apt-get update && apt-get install -y --no-install-recommends \
#     ca-certificates && \
#     rm -rf /var/lib/apt/lists/*

# # Set working directory
# WORKDIR /app

# # Copy compiled binary from builder stage
# COPY --from=builder /app/target/release/server /app/server

# Expose port
EXPOSE 8080

# Run the application
CMD ["/app/server"]
