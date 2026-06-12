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

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY docker/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
