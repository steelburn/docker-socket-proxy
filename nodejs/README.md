# Docker Socket Proxy (Node.js)

The Node.js implementation of the Docker Socket Proxy using Express and http-proxy to forward HTTP requests to the Docker Unix socket at `/var/run/docker.sock`, with optional API key authentication.

## 🚀 Features

- **Express-based**: Lightweight HTTP server with middleware support
- **http-proxy**: Efficient request forwarding to Docker socket
- **API Key Authentication**: Optional authentication for security
- **Client IP Logging**: Logs incoming requests with client IP
- **Container-Ready**: Optimized for Docker deployment

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
  ghcr.io/steelburn/docker-socket-proxy:nodejs-latest
```

### Local Development

Build using the Makefile:
```bash
make build-nodejs
```

Or manually:
```bash
npm install
```

Run locally:
```bash
PORT=3277 node src/index.js
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

## 🛠️ Development

### Prerequisites

- Node.js 18+
- Docker and Docker Compose

### Building

```bash
make build-nodejs
# Or: npm install
```

### Linting

```bash
make lint-nodejs
# Or: npx eslint src/
```

### Docker Build

```bash
make docker-build-nodejs
# Or: docker build -t docker-socket-proxy:nodejs-local .
```

## 🔒 Security

- **Read-Only Socket Mount**: Container cannot modify the Docker socket
- **Optional Authentication**: API key protection for production use

If you get permission errors accessing the socket:
- Test with `--user 0` (run as root)
- For production: `--group-add $(stat -c '%g' /var/run/docker.sock)` to match socket group

## 📊 Logging

Request logging includes client IP:
```
INFO: Proxying request: GET /version from 127.0.0.1
```

## 🚢 CI/CD

CI builds multi-platform images (AMD64 and ARM64):
- `ghcr.io/steelburn/docker-socket-proxy:nodejs-latest` — latest from `main`
- `ghcr.io/steelburn/docker-socket-proxy:nodejs-<sha>` — commit-specific

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes and run `make lint-nodejs && make build-nodejs`
4. Test with Docker Compose
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.