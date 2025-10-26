# Makefile for common build and publish tasks

IMAGE ?= ghcr.io/$(shell whoami)/docker-socket-proxy
TAG ?= latest
DOCKERHUB_REPO ?= your-dockerhub-username/docker-socket-proxy

.PHONY: all build docker-build docker-push-ghcr docker-push-dockerhub clean

all: build

# Local static build (produces ./docker-socket-proxy)
build:
	CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags "-s -w" -o docker-socket-proxy main.go

docker-build:
	docker build -t $(IMAGE):$(TAG) .

# Push to GitHub Container Registry
# Requires: echo "$GITHUB_TOKEN" | docker login ghcr.io -u $GITHUB_ACTOR --password-stdin
docker-push-ghcr: docker-build
	docker push $(IMAGE):$(TAG)

# Push to Docker Hub (requires DOCKERHUB_USERNAME/DOCKERHUB_TOKEN env vars)
docker-push-dockerhub:
	docker build -t $(DOCKERHUB_REPO):$(TAG) .
	@echo "Login to Docker Hub with DOCKERHUB_USERNAME/DOCKERHUB_TOKEN exported, then run: docker push $(DOCKERHUB_REPO):$(TAG)"

clean:
	rm -f docker-socket-proxy
