# Docker Socket Proxy Monorepo

This repository contains three implementations of a Docker socket proxy, built using NodeJS, Rust, and Go. Each implementation is located in its respective directory and can be built and run independently.

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

## CI/CD

The project includes a CI configuration located in `.github/workflows/ci.yml`, which automates the building and pushing of Docker images for all three implementations. The images are tagged as follows:
- `:go-latest` for the Go version
- `:rust-latest` for the Rust version
- `:nodejs-latest` for the NodeJS version

## License

This project is licensed under the terms of the [LICENSE](LICENSE) file.