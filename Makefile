# Optional overrides from .env (see .env.example)
-include .env

IMAGE    ?= blog
TAG      ?= $(or $(shell git describe --tags --abbrev=0 2>/dev/null | sed 's/^v//'),latest)
SITE_URL ?= http://localhost:3000

.PHONY: build clean lint fmt serve docker-build

# cargo check first so build.rs regenerates sitemap/robots before
# trunk's asset pipeline copies them.
build:
	SITE_URL=$(SITE_URL) cargo check --target wasm32-unknown-unknown
	SITE_URL=$(SITE_URL) trunk build --release

serve:
	trunk serve

lint:
	cargo fmt --all -- --check
	cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings

fmt:
	cargo fmt --all

clean:
	cargo clean
	rm -rf dist

docker-build:
	docker build --platform linux/amd64 --build-arg SITE_URL=$(SITE_URL) -t $(IMAGE):$(TAG) .
