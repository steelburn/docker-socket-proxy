# Multi-stage build: build with Go, produce a small static binary and copy to scratch
FROM golang:1.25 AS builder

WORKDIR /src

# Copy everything (this repo is tiny: single-file app)
COPY . .

# Build a statically-linked binary for the target platform
# Disable Go modules for this build (module file is not present in this subfolder)
ARG TARGETPLATFORM
RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    GOARCH=amd64; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    GOARCH=arm64; \
    else \
    GOARCH=amd64; \
    fi && \
    CGO_ENABLED=0 GOOS=linux GOARCH=$GOARCH GO111MODULE=off go build -ldflags "-s -w" -o /docker-socket-proxy ./cmd/docker-socket-proxy

### Final image: scratch
FROM scratch

# Copy the built binary from the builder stage
COPY --from=builder /docker-socket-proxy /docker-socket-proxy

# Default listen port in code is 3277; expose for documentation
EXPOSE 3277

# Run as non-root numeric user (no passwd entry in scratch; numeric is fine)
USER 65532:65532

ENTRYPOINT ["/docker-socket-proxy"]
