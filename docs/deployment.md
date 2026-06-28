# Deployment

The blog is a fully static site: Leptos compiled to WebAssembly with Trunk, posts
embedded at build time. The build output in `dist/` is plain files — `.html`,
`.wasm`, `.js`, `.css`, and copied assets — that any static host can serve.

There are two supported deploy targets:

1. **Static hosting** (cPanel / Apache, Netlify, S3, …) — upload `dist/`.
2. **Docker** — a tiny container running [`static-web-server`], typically behind
   a reverse proxy (HAProxy) that terminates TLS. This is the path used in
   production.

See [`environment.md`](./environment.md) for every environment variable.

---

## 1. Building `dist/`

```bash
make build                      # uses SITE_URL (default http://localhost:3000)
SITE_URL=https://example.com make build
```

`make build` runs `cargo check` first on purpose: `build.rs` parses the posts in
`content/blog/*.mdx` and generates `static/sitemap.xml`, `static/robots.txt`,
and `static/feed.xml` from `SITE_URL`. Trunk's asset pipeline copies those files
in parallel with the cargo build, so the check has to run first or Trunk grabs a
stale copy. **Always set `SITE_URL` to the real public URL before a production
build** — it is baked into the sitemap, robots, RSS feed, and canonical tags.

The result is written to `dist/`.

### Static hosting (cPanel / Apache)

Upload the **contents** of `dist/` to the web root. `dist/.htaccess` is included
and handles the SPA fallback (every unknown path serves `index.html` so
client-side routes like `/blog/:slug` work on refresh).

---

## 2. Docker

The image is multi-stage:

- **builder** — `rust:1`, installs the `wasm32-unknown-unknown` target and the
  pinned Trunk binary, then runs `cargo check && trunk build --release`. Cargo
  registry, build artifacts, and Trunk's tool downloads are cached via BuildKit
  cache mounts, so rebuilds are fast.
- **runtime** — [`static-web-server`] on a scratch base. Final image is ~13 MB
  (vs ~50 MB for the previous nginx image). It serves `/public` over plain HTTP
  on port 80 with SPA fallback, compression, cache-control headers, and CORS —
  all configured through `SERVER_*` environment variables (see
  [`environment.md`](./environment.md)).

### Build the image

```bash
# via Makefile (tags IMAGE:TAG, defaults blog:<latest-git-tag>)
make image

# or directly
docker build --build-arg SITE_URL=https://example.com -t blog:latest .
```

`SITE_URL` is a **build arg** here, not a runtime variable — it is compiled into
the static files, so it must be set at build time.

### Run it

```bash
docker run -d -p 8080:80 blog:latest
# http://localhost:8080
```

Override runtime behaviour with `-e`, e.g. restrict CORS:

```bash
docker run -d -p 8080:80 \
  -e SERVER_CORS_ALLOW_ORIGINS="https://example.com,https://www.example.com" \
  blog:latest
```

---

## 3. Behind a reverse proxy

In production the container runs as a **backend** and the reverse proxy
(HAProxy, Nginx, Caddy, …) terminates TLS and forwards traffic to it. Publish no
host ports on the container — instead put it on the same network as the proxy
and point the proxy's backend at the container's port 80.

### Example HAProxy backend

```haproxy
frontend https-in
    bind *:443 ssl crt /etc/haproxy/certs/example.com.pem
    default_backend blog

backend blog
    server blog1 blog:80 check
```

HAProxy (or any reverse proxy) only forwards traffic — `static-web-server`
inside the container is what actually serves the files.

### Multiple domains

`static-web-server` serves regardless of the `Host` header, so serving the same
site on several domains works with no extra config. The only domain-specific
concern is **CORS**: list every origin that makes cross-origin requests via
`-e SERVER_CORS_ALLOW_ORIGINS="…"`, or leave it as `*`. Note that `SITE_URL`
stays a single value — canonical tags, the sitemap, and the RSS feed must point
at one primary domain for SEO.

---

## 4. CI / release pipeline

`.github/workflows/ci.yml`:

- **lint** — `cargo fmt --check` + `clippy -D warnings` (run both locally before
  pushing).
- **build** — `make build`, uploads `dist/` as an artifact. `SITE_URL` comes
  from the `SITE_URL` repository variable (falls back to localhost).
- **docker** — runs **only on a `vX.Y.Z` tag**. Builds for `linux/amd64` and
  pushes to GHCR.

Cut a release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This publishes `ghcr.io/<owner>/blog` tagged `0.1.0`, `0.1`, and `latest`.

> If the GHCR push fails with 403, enable
> **Settings → Actions → General → Workflow permissions → "Read and write
> permissions"** on the repository.

[`static-web-server`]: https://static-web-server.net/
