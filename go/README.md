# Docker Socket Proxy (Go)

A tiny HTTP-to-Unix-socket proxy that exposes the Docker daemon API over HTTP.
It forwards incoming HTTP requests to the Docker daemon socket at `/var/run/docker.sock`.

Key points
- Single-file Go program: `main.go`
- Bridges HTTP -> Docker Unix socket using a custom `http.Transport` that dials the socket
- Optional API key: set `API_KEY` to have the proxy add `x-api-key` to forwarded requests

Requirements
- Linux host with Docker socket available at `/var/run/docker.sock`
- Docker (for image build/run) or Go >= 1.25 to build locally

Quick run (recommended: build image and run)

Build the image (multi-stage; final image is `scratch`):
```bash
docker build -t docker-socket-proxy:local .
```

Run the container and expose the proxy port (default: 3277). Mount the Docker socket so the proxy can reach the daemon:
```bash
docker run --rm -p 3277:3277 \
  -v /var/run/docker.sock:/var/run/docker.sock:ro \
  -e API_KEY=secret \
  docker-socket-proxy:local
```

Quick test (once the container is running):
```bash
curl http://localhost:3277/version
```

If you hit a permission error when the proxy tries to dial the socket (for example: "connect: permission denied"), running the container as root is a quick way to verify the problem and will work:

```bash
docker run --rm -p 3277:3277 \
  -v /var/run/docker.sock:/var/run/docker.sock:ro \
  -e API_KEY=secret \
  --user 0 \
  docker-socket-proxy:local
```

Local build without Docker (if you have Go >= 1.25):
```bash
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags "-s -w" -o docker-socket-proxy main.go
```

If you don't want to upgrade your local Go, you can do a one-off containerized build:
```bash
docker run --rm -v "$PWD":/src -w /src golang:1.25 \
  bash -lc 'CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags "-s -w" -o docker-socket-proxy main.go'
```

Environment variables
-- `PORT` — port the proxy listens on (default: `3277`)
-- `API_KEY` — optional; when set the proxy adds an `x-api-key` header to forwarded requests

Note: `--user 0` (run as root) is a quick way to bypass socket permission issues for testing. For production the recommended approaches are:

- Use `--group-add $(stat -c '%g' /var/run/docker.sock)` so the container process is in the same group as the socket (keeps non-root runtime).
- Or create a small entrypoint that maps the host socket GID to a group inside the container and drops privileges. Avoid `chmod 666 /var/run/docker.sock` in production.

Security & permissions
- The proxy exposes the full Docker HTTP API. Only run it in trusted environments.
- The image's final `scratch` stage uses numeric user `65532:65532` to avoid running as root.
  Depending on the Docker socket permissions on your host, you may need to run the container as root or adjust socket perms. If you get permission errors, run with `--user 0` or adjust the image/user.
- The Docker socket is a privileged interface. Prefer using a host firewall, network isolation, and API_KEY to control access.

Extending the proxy
- Add headers, auth, or request filtering in `proxy.Director` in `main.go`.
- Add middleware by wrapping the `proxy` handler when creating the HTTP server.

Notes
- This repository intentionally contains a tiny, dependency-free implementation (standard library only).
- The Dockerfile builds a statically-linked binary and places it into a `scratch` final image for minimal attack surface and small image size.

Images & tagging
- CI builds for this Go repository tag images to avoid colliding with other language variants. Images built by the CI are tagged as:
  - `ghcr.io/steelburn/docker-socket-proxy:go-latest` — the latest Go build from `main`
  - `ghcr.io/steelburn/docker-socket-proxy:go-<sha>` — the commit-specific image
  
  When a GitHub Release is published, the release workflow publishes:
  - `ghcr.io/steelburn/docker-socket-proxy:go-<release-tag>` (e.g. `go-v1.2.3`)
  - `ghcr.io/steelburn/docker-socket-proxy:go-latest`

  You can override the image name when building locally with the `Makefile`:
  ```bash
  make IMAGE=ghcr.io/your-org/docker-socket-proxy TAG=go-v1.2.3 docker-build
  ```

License
- No license file present in the repository. Add one if you intend to publish or share widely.
