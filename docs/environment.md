# Environment variables

There are two distinct groups of variables, and the distinction matters:

- **Build-time** — read while compiling the site. Their values are *baked into
  the static files* and cannot be changed afterwards without rebuilding.
- **Runtime** — read by `static-web-server` when the container starts. Change
  them by restarting the container; no rebuild needed.

Copy [`.env.example`](../.env.example) to `.env` to set the build-time and
compose variables locally — the `Makefile` and `docker-compose.yml` both read
it automatically.

---

## Build-time

| Variable   | Used by                          | Default                 | Description                                                                                                                                            |
| ---------- | -------------------------------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `SITE_URL` | `build.rs`, `make build`, Docker `--build-arg` | `http://localhost:3000` | Public base URL. Baked into `sitemap.xml`, `robots.txt`, `feed.xml`, and per-page canonical / Open Graph tags. **Single value** — set it to the real production domain before a release build. |

Why single-valued: canonical URLs, the sitemap, and the RSS feed must point at
one primary domain or search engines see duplicate content. Serving the same
build on several domains is fine — that's a CORS concern (below), not a build
concern.

### Compose / Makefile helpers

These configure *how* the build and image are produced; they are not consumed by
the application itself.

| Variable       | Used by                | Default                       | Description                                                                 |
| -------------- | ---------------------- | ----------------------------- | --------------------------------------------------------------------------- |
| `IMAGE`        | `make docker-build`    | `blog`                        | Docker image name.                                                          |
| `TAG`          | `make docker-build`    | latest git tag minus `v`, else `latest` | Docker image tag.                                                |
| `CORS_ORIGINS` | `docker-compose.yml`   | `*`                           | Passed through to the container as `SERVER_CORS_ALLOW_ORIGINS` (see below). |

---

## Runtime (`static-web-server`)

Set in the `runtime` stage of the `Dockerfile`. Override any of them with `-e`
on `docker run` or under `environment:` in `docker-compose.yml`. Full reference:
<https://static-web-server.net/configuration/environment-variables/>.

| Variable                       | Value in image          | Description                                                                                                   |
| ------------------------------ | ----------------------- | ------------------------------------------------------------------------------------------------------------- |
| `SERVER_ROOT`                  | `/public`               | Directory served (the copied `dist/`).                                                                        |
| `SERVER_PORT`                  | `80`                    | Listen port inside the container.                                                                             |
| `SERVER_FALLBACK_PAGE`         | `/public/index.html`    | SPA fallback — served with `200` for unknown paths so client-side routes (`/blog/:slug`) work on refresh.     |
| `SERVER_COMPRESSION`           | `true`                  | gzip / deflate / brotli / zstd based on `Accept-Encoding`.                                                    |
| `SERVER_CACHE_CONTROL_HEADERS` | `true`                  | Auto `Cache-Control`: ~1 year for hashed assets (css/js), ~1 day for documents (html/wasm).                   |
| `SERVER_CORS_ALLOW_ORIGINS`    | `*`                     | Comma-separated list of allowed origins, or `*` for any. Set this to your domains when serving cross-origin.  |
| `SERVER_CORS_ALLOW_HEADERS`    | `origin, content-type, accept, range` | Allowed request headers for CORS.                                                              |
| `SERVER_CORS_EXPOSE_HEADERS`   | `content-length, content-range`       | Response headers exposed to the browser.                                                       |

### CORS and multiple domains

`static-web-server` serves regardless of the request `Host`, so the same image
works on any number of domains with no change. The only per-domain setting is
which origins may make **cross-origin** requests:

```yaml
# docker-compose.yml — restrict to your domains
environment:
  SERVER_CORS_ALLOW_ORIGINS: "https://example.com,https://www.example.com"
```

or via the `.env` shortcut that `docker-compose.yml` reads:

```dotenv
CORS_ORIGINS=https://example.com,https://www.example.com
```

Leaving it as `*` (the default) allows any origin, which is fine for a public
static blog and its RSS feed.

---

## Quick reference

```dotenv
# .env  (copy from .env.example)

# build-time: real public domain (baked into sitemap/robots/feed/canonical)
SITE_URL=https://example.com

# docker image name/tag for `make docker-build`
IMAGE=blog
#TAG=0.1.0

# runtime CORS origins for docker-compose (defaults to *)
#CORS_ORIGINS=https://example.com,https://www.example.com
```
