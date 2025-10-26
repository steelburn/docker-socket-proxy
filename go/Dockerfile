# Multi-stage build: build with Go, produce a small static binary and copy to scratch
FROM golang:1.25 AS builder

WORKDIR /src

# Build a statically-linked binary for Linux AMD64
ENV CGO_ENABLED=0 GOOS=linux GOARCH=amd64

# Copy everything (this repo is tiny: single-file app)
COPY . .

# Build the binary (strip debug info for smaller size)
RUN go build -ldflags "-s -w" -o /docker-socket-proxy main.go

### Final image: scratch
FROM scratch

# Copy the built binary from the builder stage
COPY --from=builder /docker-socket-proxy /docker-socket-proxy

# Default listen port in code is 3277; expose for documentation
EXPOSE 3277

# Run as non-root numeric user (no passwd entry in scratch; numeric is fine)
USER 65532:65532

ENTRYPOINT ["/docker-socket-proxy"]
