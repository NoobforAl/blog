# Optional overrides from .env (see .env.example)
-include .env

IMAGE    ?= blog
TAG      ?= $(or $(shell git describe --tags --abbrev=0 2>/dev/null | sed 's/^v//'),latest)
SITE_URL ?= http://localhost:3000

.PHONY: dev build image clean

# Dev server with hot reload on http://localhost:3000 (rebuilds on file change).
dev:
	trunk serve

# Production build -> dist/. Trunk's pre_build hook (see Trunk.toml) runs
# build.rs first so the generated sitemap/robots/feed are copied into dist/.
build:
	SITE_URL=$(SITE_URL) trunk build --release

# Build the Docker image (tags IMAGE:TAG, defaults blog:<latest-git-tag>).
image:
	docker build --platform linux/amd64 --build-arg SITE_URL=$(SITE_URL) -t $(IMAGE):$(TAG) .

clean:
	cargo clean
	rm -rf dist
