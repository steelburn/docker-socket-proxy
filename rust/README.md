# Docker Socket Proxy (Rust)

A minimal, secure, and high-performance HTTP proxy for the Docker Unix socket, written in Rust. This proxy enables remote access to the Docker API while maintaining security through Unix socket isolation and optional API key authentication.

## 🚀 Features

- **Minimal & Secure**: Single-binary application with no external dependencies in production
- **Streaming Proxy**: Zero-copy request/response streaming for optimal performance
- **Unix Socket Communication**: Direct communication with Docker daemon via `/var/run/docker.sock`
- **Optional Authentication**: API key-based authentication for added security
- **Client IP Logging**: Logs incoming requests with client IP for debugging
- **Container-Ready**: Optimized for containerized deployment with scratch-based images
- **Multi-Platform**: Supports AMD64 and ARM64 architectures

## 📦 Quick Start

### Using Docker Compose (Recommended)

From the repository root:
```bash
cp .env.sample .env
# Edit .env as needed
docker-compose up -d
```

### Using Pre-built Images

```bash
docker run --rm -p 3277:3277 \
  -v /var/run/docker.sock:/var/run/docker.sock:ro \
  -e API_KEY=secret \
  --user 0 \
  ghcr.io/steelburn/docker-socket-proxy:rust-latest
```

**Note**: The `--user 0` flag runs the container as root, which is required for Docker socket access.

### Local Development

Build using the Makefile:
```bash
make build-rust
```

Or manually:
```bash
cargo build --release
```

Run locally:
```bash
PORT=3277 cargo run
```

Test:
```bash
curl http://localhost:3277/version
```

## 🔧 Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3277` | Port the proxy listens on |
| `API_KEY` | (none) | Optional API key for authentication |
| `RUST_LOG` | `actix_web=info,docker_socket_proxy=info` | Logging level configuration |

## 🛠️ Development

### Prerequisites

- Rust 1.90+
- Docker and Docker Compose
- Linux/Unix environment (for Docker socket access)

### Building

```bash
make build-rust
# Or: cargo build --release
```

### Linting

```bash
make lint-rust
# Or: cargo clippy
```

### Docker Build

```bash
make docker-build-rust
# Or: docker build -t docker-socket-proxy:rust-local .
```

## 🔒 Security

- **Unix Socket Isolation**: Leverages Docker's built-in security boundary
- **Read-Only Socket Mount**: Container cannot modify the Docker socket
- **No-New-Privileges**: Prevents privilege escalation
- **Minimal Attack Surface**: Scratch-based image with single static binary
- **Optional Authentication**: API key protection for production use

If you get permission errors accessing the socket:
- Test with `--user 0` (run as root)
- For production: `--group-add $(stat -c '%g' /var/run/docker.sock)` to match socket group

## 📊 Logging

### Log Levels

```bash
# Debug mode (verbose)
RUST_LOG=actix_web=debug,docker_socket_proxy=debug

# Production mode
RUST_LOG=actix_web=warn,docker_socket_proxy=info
```

Request logging includes client IP:
```
INFO: Proxying request: GET /version from 127.0.0.1
```

## 🚢 CI/CD

CI builds multi-platform images (AMD64 and ARM64) with GitHub Actions cache:
- `ghcr.io/steelburn/docker-socket-proxy:rust-latest` — latest from `main`
- `ghcr.io/steelburn/docker-socket-proxy:rust-<sha>` — commit-specific

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes and run `make lint-rust && make build-rust`
4. Test with Docker Compose
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.