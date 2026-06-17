# syntax=docker/dockerfile:1
FROM rust:1 AS builder

RUN rustup target add wasm32-unknown-unknown
ADD https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz /tmp/trunk.tar.gz
RUN tar -xzf /tmp/trunk.tar.gz -C /usr/local/bin && chmod +x /usr/local/bin/trunk

WORKDIR /app
COPY . .

ARG SITE_URL=http://localhost:3000
ENV SITE_URL=${SITE_URL}

# Cache mounts: cargo registry, build artifacts, and trunk's tool downloads
# (tailwindcss, wasm-bindgen) survive across builds.
# cargo check first: build.rs must write static/sitemap.xml + robots.txt
# before trunk's asset pipeline copies them (it runs parallel to the build).
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/root/.cache/trunk \
    cargo check --target wasm32-unknown-unknown && trunk build --release

# static-web-server: tiny (~2.5 MB, scratch-based) Rust static file server.
# Sits behind the external reverse proxy (HAProxy), serving plain HTTP internally.
FROM joseluisq/static-web-server:2 AS runtime

COPY --from=builder /app/dist /public

ENV SERVER_ROOT=/public \
    SERVER_PORT=80 \
    SERVER_FALLBACK_PAGE=/public/index.html \
    SERVER_COMPRESSION=true \
    SERVER_CACHE_CONTROL_HEADERS=true \
    # CORS: default allows any origin (fine for a public static blog / RSS feed).
    # To restrict, override with a comma-separated list of your domains, e.g.
    # SERVER_CORS_ALLOW_ORIGINS="https://example.com,https://www.example.com"
    SERVER_CORS_ALLOW_ORIGINS=* \
    SERVER_CORS_ALLOW_HEADERS="origin, content-type, accept, range" \
    SERVER_CORS_EXPOSE_HEADERS="content-length, content-range"

EXPOSE 80
