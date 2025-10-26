# Build stage
FROM rust:1.90.0-alpine3.22 AS builder

# Install build dependencies (gcc, musl, pkgconfig, openssl dev for crates using native libs)
RUN apk add --no-cache musl-dev build-base pkgconf openssl-dev ca-certificates

# Install Rust targets for multi-platform builds
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl

# Set working directory
WORKDIR /usr/src/app

# Copy dependency files first for better layer caching
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies for the target architecture
# Use build arg to determine target
ARG TARGETPLATFORM
RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    TARGET="x86_64-unknown-linux-musl"; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    TARGET="aarch64-unknown-linux-musl"; \
    else \
    TARGET="x86_64-unknown-linux-musl"; \
    fi && \
    cargo build --release --target $TARGET && rm -rf src

# Copy source code
COPY src ./src

# Build the application for the target architecture
ARG TARGETPLATFORM
RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    TARGET="x86_64-unknown-linux-musl"; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    TARGET="aarch64-unknown-linux-musl"; \
    else \
    TARGET="x86_64-unknown-linux-musl"; \
    fi && \
    touch src/main.rs && cargo build --release --target $TARGET

# Binary selection stage
FROM alpine:latest AS binary-selector

ARG TARGETPLATFORM
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/docker_socket_proxy /amd64/

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    cp /amd64/docker_socket_proxy /docker_socket_proxy; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    echo "Arm64 binary not built for single-platform build"; exit 1; \
    fi

# Runtime stage - using scratch for minimal image
FROM scratch

# Copy the selected binary
COPY --from=binary-selector /docker_socket_proxy /

# Expose default port
EXPOSE 3277

# Set default environment variables
ENV PORT=3277
ENV RUST_LOG=actix_web=debug

# Run the application
CMD ["/docker_socket_proxy"]