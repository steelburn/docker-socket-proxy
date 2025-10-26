# Docker Socket Proxy Monorepo

This repository contains three implementations of a Docker socket proxy, built using NodeJS, Rust, and Go. Each implementation is located in its respective directory and can be built and run independently.

## Features

- **Multi-language implementations**: Go, Rust, and Node.js versions with identical functionality
- **Docker socket proxying**: Forwards HTTP requests to the Docker daemon via Unix socket
- **API key authentication**: Optional authentication via `API_KEY` environment variable
- **Client IP logging**: Logs incoming requests with client IP for debugging
- **Multi-platform Docker images**: AMD64 and ARM64 support
- **Comprehensive tooling**: Makefile, Docker Compose, linting, and CI/CD

## Prerequisites

- Docker and Docker Compose
- Make (for build orchestration)
- For local development:
  - Go 1.25+ (for Go implementation)
  - Rust 1.90+ (for Rust implementation)
  - Node.js 18+ (for Node.js implementation)

## Quick Start

### Using Docker Compose (Recommended)

1. Copy the environment template:
   ```bash
   cp .env.sample .env
   ```

2. Edit `.env` to set your configuration:
   ```bash
   PORT=3277
   API_KEY=your-secret-api-key  # Optional
   ```

3. Start the proxy:
   ```bash
   docker-compose up -d
   ```

The proxy will be available at `http://localhost:3277` and forwards requests to the Docker socket.

### Using Pre-built Images

```bash
# Choose your implementation
docker run -d \
  -p 3277:3277 \
  -v /var/run/docker.sock:/var/run/docker.sock:ro \
  -e API_KEY=your-secret \
  ghcr.io/steelburn/docker-socket-proxy:go-latest
```

Available tags: `go-latest`, `rust-latest`, `nodejs-latest`

### Local Development

1. Build all implementations:
   ```bash
   make build-all
   ```

2. Or build specific language:
   ```bash
   make build-go
   make build-rust
   make build-nodejs
   ```

3. Run locally (example for Go):
   ```bash
   cd go && PORT=3277 ./docker-socket-proxy
   ```

## Project Structure

- **go/**: Contains the Go implementation of the Docker socket proxy.
  - **Dockerfile**: Instructions to build the Docker image for the Go version.
  - **go.mod**: Module definition and dependencies for the Go application.
  - **cmd/docker-socket-proxy/main.go**: Entry point for the Go application.
  - **README.md**: Documentation specific to the Go implementation.

- **rust/**: Contains the Rust implementation of the Docker socket proxy.
  - **Dockerfile**: Instructions to build the Docker image for the Rust version.
  - **Cargo.toml**: Package definition and dependencies for the Rust application.
  - **src/main.rs**: Entry point for the Rust application.
  - **README.md**: Documentation specific to the Rust implementation.

- **nodejs/**: Contains the NodeJS implementation of the Docker socket proxy.
  - **Dockerfile**: Instructions to build the Docker image for the NodeJS version.
  - **package.json**: Defines the NodeJS application, including dependencies and scripts.
  - **src/index.js**: Entry point for the NodeJS application.
  - **README.md**: Documentation specific to the NodeJS implementation.

## Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| `PORT` | `3277` | Port the proxy listens on |
| `API_KEY` | (none) | Optional API key for authentication |

## Development

### Building

Use the Makefile for consistent builds across all implementations:

```bash
# Build everything
make build-all

# Build specific language
make build-go
make build-rust
make build-nodejs

# Clean builds
make clean-all

# Docker builds
make docker-build-all
```

### Testing

```bash
# Run tests (when implemented)
make test-all

# Lint code
make lint-all

# Run all checks
make lint-all && make test-all && make build-all
```

### Linting

The project uses language-specific linters:
- **Go**: golangci-lint
- **Rust**: cargo clippy
- **Node.js**: ESLint

Run `make lint-all` or check individual languages.

## CI/CD

The project includes comprehensive CI/CD:

- **Build workflow** (`.github/workflows/docker-image-build.yml`): Multi-platform Docker builds with caching
- **Lint workflow** (`.github/workflows/lint.yml`): Code quality checks on pushes/PRs
- **Dependabot** (`.github/dependabot.yml`): Automated dependency updates

Images are pushed to GitHub Container Registry and tagged as follows:
- `ghcr.io/steelburn/docker-socket-proxy:go-latest` and `go-<commit-sha>` for the Go version
- `ghcr.io/steelburn/docker-socket-proxy:rust-latest` and `rust-<commit-sha>` for the Rust version
- `ghcr.io/steelburn/docker-socket-proxy:nodejs-latest` and `nodejs-<commit-sha>` for the NodeJS version

## API Usage

The proxy forwards all HTTP requests to the Docker daemon. Example:

```bash
# List containers
curl http://localhost:3277/containers/json

# With API key
curl -H "x-api-key: your-secret" http://localhost:3277/containers/json
```

## AI Guidance

For AI coding assistants, refer to `.github/copilot-instructions.md` for project-specific conventions, build patterns, and implementation details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes and run `make lint-all && make build-all`
4. Test with Docker Compose
5. Submit a pull request

## License

This project is licensed under the terms of the [LICENSE](LICENSE) file.