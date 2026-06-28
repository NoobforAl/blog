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
# Trunk's pre_build hook (Trunk.toml) runs build.rs before the asset pipeline so
# the generated static/sitemap.xml, robots.txt and feed.xml are copied into dist/.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/root/.cache/trunk \
    trunk build --release

# static-web-server: tiny (~2.5 MB, scratch-based) Rust static file server.
# Serves the built dist/ over plain HTTP on port 80 (put a reverse proxy in
# front of it to terminate TLS).
FROM joseluisq/static-web-server:2 AS runtime

COPY --from=builder /app/dist /public

# CORS defaults to any origin (fine for a public static blog / RSS feed). To
# restrict, override SERVER_CORS_ALLOW_ORIGINS at run time with a comma-separated
# list, e.g. -e SERVER_CORS_ALLOW_ORIGINS="https://example.com,https://www.example.com"
ENV SERVER_ROOT=/public \
    SERVER_PORT=80 \
    SERVER_FALLBACK_PAGE=/public/index.html \
    SERVER_COMPRESSION=true \
    SERVER_CACHE_CONTROL_HEADERS=true \
    SERVER_CORS_ALLOW_ORIGINS=* \
    SERVER_CORS_ALLOW_HEADERS="origin, content-type, accept, range" \
    SERVER_CORS_EXPOSE_HEADERS="content-length, content-range"

EXPOSE 80
