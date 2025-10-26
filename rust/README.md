# Docker Socket Proxy

A minimal, secure, and high-performance HTTP proxy for the Docker Unix socket, written in Rust. This proxy enables remote access to the Docker API while maintaining security through Unix socket isolation and optional API key authentication.

## 🚀 Features

- **Minimal & Secure**: Single-binary application with no external dependencies in production
- **Streaming Proxy**: Zero-copy request/response streaming for optimal performance
- **Unix Socket Communication**: Direct communication with Docker daemon via `/var/run/docker.sock`
- **Optional Authentication**: API key-based authentication for added security
- **Verbose Logging**: Comprehensive logging for debugging and monitoring
- **Container-Ready**: Optimized for containerized deployment with scratch-based images
- **Multi-Platform**: Supports AMD64 and ARM64 architectures

## 🏗️ Architecture

```
Client Request → Actix-Web Server → Unix Socket → Docker Daemon
                      ↓
                Response Stream ← Unix Socket ← Docker Daemon
```

The proxy acts as a transparent HTTP proxy, forwarding all requests to the Docker Unix socket while preserving headers and streaming response bodies without buffering.

## 📦 Quick Start

### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/steelburn/docker-socket-proxy-rust.git
cd docker-socket-proxy-rust

# Start the proxy
docker-compose up -d

# Test the proxy
curl http://localhost:8080/version
```

### Manual Docker Run

```bash
# Build the image
docker build -t docker-socket-proxy .

# Run with Docker socket mounted
docker run -d \
  --name docker-socket-proxy \
  --user 0 \
  -p 8080:3277 \
  -v /var/run/docker.sock:/var/run/docker.sock:ro \
  -e RUST_LOG=actix_web=info,docker_socket_proxy=info \
  docker-socket-proxy
```

## 🔧 Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3277` | Port the proxy listens on |
| `API_KEY` | (none) | Optional API key for authentication |
| `RUST_LOG` | `actix_web=info` | Logging level configuration |

### Docker Compose Configuration

```yaml
services:
  docker-socket-proxy:
    build: .
    ports:
      - "8080:3277"  # Host:Container port mapping
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro  # Read-only socket mount
    environment:
      - PORT=3277
      - RUST_LOG=actix_web=debug,docker_socket_proxy=debug
      - API_KEY=your-secret-key  # Optional
    security_opt:
      - no-new-privileges:true
    read_only: true
    restart: unless-stopped
```

## 🧪 Testing

### Basic Connectivity Test

```bash
# Test Docker API access through proxy
curl http://localhost:8080/version

# List containers
curl http://localhost:8080/v1.41/containers/json

# Get system info
curl http://localhost:8080/v1.41/info
```

### With API Key Authentication

```bash
# Include API key in request header
curl -H "x-api-key: your-secret-key" \
     http://localhost:8080/version
```

### Health Check

```bash
# Check if proxy is responding
curl -f http://localhost:8080/version || echo "Proxy is down"
```

## 🛠️ Development

### Prerequisites

- Rust 1.82 or later
- Docker and Docker Compose
- Linux/Unix environment (for Docker socket access)

### Local Development Setup

```bash
# Clone and build
git clone https://github.com/steelburn/docker-socket-proxy-rust.git
cd docker-socket-proxy-rust

# Build and run locally
cargo build --release
PORT=8080 cargo run

# Or run with Docker for socket access
docker-compose up --build
```

### Testing with Real Docker API

```bash
# Start a test container
docker run -d --name test-container nginx:alpine

# Query through proxy
curl "http://localhost:8080/v1.41/containers/json" | jq '.[] | select(.Names[0] == "/test-container")'

# Clean up
docker rm -f test-container
```

## 🔒 Security

### Security Features

- **Unix Socket Isolation**: Leverages Docker's built-in security boundary
- **Read-Only Socket Mount**: Container cannot modify the Docker socket
- **No-New-Privileges**: Prevents privilege escalation
- **Minimal Attack Surface**: Scratch-based image with single static binary
- **Optional Authentication**: API key protection for production use

### Production Deployment

```yaml
# Example production docker-compose.yml
services:
  docker-socket-proxy:
    image: ghcr.io/steelburn/docker-socket-proxy-rust:rust-latest
    user: "0"
    ports:
      - "127.0.0.1:8080:3277"  # Bind to localhost only
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
    environment:
      - API_KEY=${DOCKER_PROXY_API_KEY}  # Use environment variable
      - RUST_LOG=actix_web=warn,docker_socket_proxy=info
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
    restart: unless-stopped
    networks:
      - internal
```

## 📊 Monitoring & Logging

### Log Levels

```bash
# Debug mode (verbose)
RUST_LOG=actix_web=debug,docker_socket_proxy=debug

# Production mode
RUST_LOG=actix_web=warn,docker_socket_proxy=info

# Minimal logging
RUST_LOG=actix_web=error,docker_socket_proxy=error
```

### Startup Logs

```
🚀 Starting docker_socket_proxy v0.1.0
📡 Binding to 0.0.0.0:3277
✅ docker_socket_proxy v0.1.0 is ready and listening on port 3277
🔗 Docker socket proxy ready to accept connections
```

### Request Logging

```
[INFO] Received GET request for /v1.41/containers/json
[DEBUG] Successfully connected to Docker socket at /var/run/docker.sock
[DEBUG] Added header: Accept: application/json
[INFO] Docker API responded with status code: 200
[INFO] Proxy request completed successfully for GET /v1.41/containers/json
```

## 🚢 CI/CD

The project includes GitHub Actions for automated building and deployment:

- **Triggers**: Pushes to `main`, pull requests, and releases
- **Platforms**: Multi-platform builds (AMD64/ARM64)
- **Registry**: Automatic push to GitHub Container Registry
- **Tagging**: Semantic versioning and branch-based tags

Images are available at:
```
ghcr.io/steelburn/docker-socket-proxy-rust:rust-latest
ghcr.io/steelburn/docker-socket-proxy-rust:v0.1.0
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes and add tests
4. Run the build: `docker-compose up --build`
5. Commit your changes: `git commit -am 'Add some feature'`
6. Push to the branch: `git push origin feature/your-feature`
7. Submit a pull request

### Development Guidelines

- Follow Rust best practices and idioms
- Add comprehensive logging for new features
- Update documentation for API changes
- Test with real Docker API calls
- Ensure security best practices are maintained

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Actix Web](https://actix.rs/) for high-performance HTTP handling
- Uses [Tokio](https://tokio.rs/) for async runtime and Unix socket support
- Inspired by the need for secure Docker API access in containerized environments

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/steelburn/docker-socket-proxy-rust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/steelburn/docker-socket-proxy-rust/discussions)
- **Documentation**: See inline code comments and this README

