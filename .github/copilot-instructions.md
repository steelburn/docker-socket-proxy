<!-- Copilot instructions for contributors and AI assistants -->
# repo: docker-socket-proxy — quick orientation for code-writing agents

This monorepo contains three independent implementations of the same service: a tiny HTTP->Docker-unix-socket proxy written in Go, Rust and NodeJS. Keep the guidance below short and actionable so an AI can be productive immediately.

Key facts
- Monorepo layout: `go/`, `rust/`, `nodejs/`. Each folder is a standalone implementation with its own Dockerfile and README.
- Default runtime port: `3277` (env var `PORT`). Optional API auth: env var `API_KEY` — implementations add `x-api-key` when set.
- Docker socket path: `/var/run/docker.sock` (read-only mount recommended).

Big-picture architecture
- Each implementation accepts HTTP requests and forwards them to Docker over the Unix socket. They differ in implementation style:
  - Go: `go/cmd/docker-socket-proxy/main.go` uses `httputil.NewSingleHostReverseProxy` and a custom `http.Transport` with a Unix socket Dial.
  - Rust: `rust/src/main.rs` uses Actix and `tokio::net::UnixStream`, manually composes raw HTTP framing and streams responses.
  - NodeJS: `nodejs/src/index.js` uses `express` + `http-proxy` with `socketPath: '/var/run/docker.sock'`.

What to change where (common edits)
- Add middleware/auth: Go -> modify `proxy.Director` in `main.go`; Node -> add Express middleware before `app.all('/*', ...)`; Rust -> modify `proxy` handler to inject headers or short-circuit.
- Add request filtering or header rewrites: follow examples in `go/cmd/...` and `rust/src/main.rs` where headers are built/forwarded.

Build / run (discoverable and tested flows)
- Build Docker image (per-language): `docker build -t docker-socket-proxy:local .` inside each language folder.
- Run container for testing: mount socket readonly and expose port, e.g.:
  - `docker run --rm -p 3277:3277 -v /var/run/docker.sock:/var/run/docker.sock:ro -e API_KEY=secret docker-socket-proxy:local`
  - If you get socket permission errors, run temporarily as root: add `--user 0` or prefer `--group-add $(stat -c '%g' /var/run/docker.sock)`.
- Local builds:
  - Go: `CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags "-s -w" -o docker-socket-proxy main.go` (see `go/README.md`).
  - Rust: `cargo build --release` (see `rust/README.md`).
  - Node: `npm install` then `node src/index.js` (see `nodejs/README.md`).
- Quick health check: `curl http://localhost:3277/version` (or `-H 'x-api-key: secret'` when `API_KEY` is enforced).

Project-specific conventions
- Port + API key envs are the canonical configuration surface (`PORT`, `API_KEY`). Keep them consistent across language changes.
- The Docker image targets minimal runtime (Go/Rust use static single-binary + scratch). Avoid introducing large runtime dependencies in final images.
- Default logging: each implementation logs requests; follow existing patterns (simple INFO/DEBUG messages). Minimal, structured logs are preferred but keep parity with current prints.

Integration & CI
- CI builds and tags language-specific images (see `.github/workflows/ci.yml` and `go/README.md` for tag conventions like `go-latest`, `rust-latest`, `nodejs-latest`).
- Image naming used in docs: `ghcr.io/steelburn/docker-socket-proxy:<lang>-latest`.

Troubleshooting notes for agents
- Common failure: permission denied when connecting to `/var/run/docker.sock`. Reproduce locally by running container with `--user 0` or adding the socket GID as a group. Point this out in PR descriptions.
- If tests fail due to missing real Docker API, prefer integration tests that run in CI with socket access or use small mocks when possible.

Files worth inspecting when implementing changes
- `go/cmd/docker-socket-proxy/main.go` — reverse-proxy implementation and `proxy.Director`.
- `rust/src/main.rs` — manual HTTP framing and streaming; useful for low-level behavior.
- `nodejs/src/index.js` — express + http-proxy pattern and simple middleware.
- `go/README.md`, `rust/README.md`, `nodejs/README.md` — run/build examples and security notes.

When creating PRs
- Reference the implementation you changed (e.g., "Go: add header filter in proxy.Director (go/cmd/...)"), include a quick repro (build+docker run) and note any socket permission guidance.

If anything here is unclear or you'd like more examples (small code diffs for adding middleware, test harness, or CI tweak), tell me which implementation(s) to prioritize and I will iterate.
