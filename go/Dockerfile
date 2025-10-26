# Multi-stage build: build with Go, produce a small static binary and copy to scratch
FROM golang:1.25 AS builder

WORKDIR /src

# Build a statically-linked binary for Linux AMD64
# Disable Go modules for this build (module file is not present in this subfolder)
ENV CGO_ENABLED=0 GOOS=linux GOARCH=amd64 GO111MODULE=off

# Copy everything (this repo is tiny: single-file app)
COPY . .

# The repository places the main package under cmd/docker-socket-proxy/main.go
# Build from that package path so the Docker build succeeds when the context is the `go/` folder.
RUN go build -ldflags "-s -w" -o /docker-socket-proxy ./cmd/docker-socket-proxy

### Final image: scratch
FROM scratch

# Copy the built binary from the builder stage
COPY --from=builder /docker-socket-proxy /docker-socket-proxy

# Default listen port in code is 3277; expose for documentation
EXPOSE 3277

# Run as non-root numeric user (no passwd entry in scratch; numeric is fine)
USER 65532:65532

ENTRYPOINT ["/docker-socket-proxy"]
