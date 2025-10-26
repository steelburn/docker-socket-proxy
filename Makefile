# Makefile for Docker Socket Proxy Monorepo

.PHONY: help build-all build-go build-rust build-nodejs clean-all clean-go clean-rust clean-nodejs docker-build-all docker-build-go docker-build-rust docker-build-nodejs test-all test-go test-rust test-nodejs lint-all lint-go lint-rust lint-nodejs

# Default target
help:
	@echo "Available targets:"
	@echo "  build-all      - Build all implementations"
	@echo "  build-go       - Build Go implementation"
	@echo "  build-rust     - Build Rust implementation"
	@echo "  build-nodejs   - Build Node.js implementation"
	@echo "  clean-all      - Clean all build artifacts"
	@echo "  docker-build-all - Build Docker images for all implementations"
	@echo "  test-all       - Run tests for all implementations"
	@echo "  lint-all       - Run linting for all implementations"

# Build targets
build-all: build-go build-rust build-nodejs

build-go:
	cd go && make build

build-rust:
	cd rust && cargo build --release

build-nodejs:
	cd nodejs && npm install

# Clean targets
clean-all: clean-go clean-rust clean-nodejs

clean-go:
	cd go && make clean

clean-rust:
	cd rust && cargo clean

clean-nodejs:
	cd nodejs && rm -rf node_modules

# Docker build targets
docker-build-all: docker-build-go docker-build-rust docker-build-nodejs

docker-build-go:
	cd go && make docker-build

docker-build-rust:
	docker build -t docker-socket-proxy:rust-local rust/

docker-build-nodejs:
	docker build -t docker-socket-proxy:nodejs-local nodejs/

# Test targets (if tests exist)
test-all: test-go test-rust test-nodejs

test-go:
	@echo "No Go tests defined yet"

test-rust:
	cd rust && cargo test

test-nodejs:
	@echo "No Node.js tests defined yet"

# Lint targets
lint-all: lint-go lint-rust lint-nodejs

lint-go:
	cd go && golangci-lint run

lint-rust:
	cd rust && cargo clippy

lint-nodejs:
	cd nodejs && npx eslint src/