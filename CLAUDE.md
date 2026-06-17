# CLAUDE.md

Personal blog + resume site written fully in Rust: Yew (CSR, WebAssembly) compiled with Trunk. No Node.js anywhere.

## Commands

```bash
trunk serve                # dev server on http://localhost:3000 (hot reload)
make build                 # production build -> dist/ (uses SITE_URL env, default http://localhost:3000)
make lint                  # cargo fmt --check + clippy (-D warnings) — same gates as CI
make fmt                   # auto-format
make clean                 # cargo clean + remove dist/
make docker-build          # build docker image (IMAGE/TAG/SITE_URL overridable)
cargo check --target wasm32-unknown-unknown   # fast compile check
```

Toolchain: stable Rust with the `wasm32-unknown-unknown` target, plus the `trunk` binary. Trunk downloads Tailwind CSS v4 and wasm-bindgen automatically (versions pinned in `Trunk.toml` / `Cargo.lock`).

## Architecture

- **Posts are compiled in.** `build.rs` parses YAML frontmatter from `content/blog/*.mdx`, generates `$OUT_DIR/posts_gen.rs` (included by `src/posts.rs`), and writes `static/sitemap.xml`, `static/robots.txt`, and `static/feed.xml` (RSS 2.0) using the `SITE_URL` env var. Adding a post = drop an `.mdx` file in `content/blog/` and rebuild. The generated static files are gitignored — never edit them by hand, and always run the build (or `cargo check`) before `trunk build` so they're fresh (trunk copies assets in parallel with the cargo build; `make build` and the Dockerfile already do this).
- `src/main.rs` — router (`/`, `/blog`, `/blog/:slug`, 404) and app shell.
- `src/profile.rs` — ALL resume/personal content for the home page (name, tagline, skills, projects). Edit content here, not in `pages/home.rs`.
- `src/meta.rs` — runtime SEO: per-route title, description, canonical, Open Graph/Twitter tags, `article:*` times. Every page sets its meta in a `use_effect_with` hook.
- `src/markdown.rs` — pulldown-cmark → raw HTML via `Html::from_html_unchecked`. Element styling comes from `.markdown-body` rules in `styles/globals.css`, not per-element classes.
- Syntax highlighting: highlight.js loaded from CDN in `index.html` (SRI-pinned); `meta::highlight_code_blocks()` re-runs it after a post renders.
- `index.html` is the Trunk entry: Tailwind input, copied static assets (`robots.txt`, `sitemap.xml`, `.htaccess`, `static/images/`), fonts (Bricolage Grotesque display, JetBrains Mono).

## Conventions & gotchas

- Tailwind v4 scans `src/` and `index.html` via `@source` in `styles/globals.css` — utility classes inside `html!` string literals are picked up. Custom classes (`container-responsive`, `hacker-bg`, `terminal-card`, `prompt-label`, `glow-card`, `rise`) are plain CSS in `globals.css`.
- Yew 0.23: `String` does not coerce into the `classes` prop — use `classes={Classes::from(string)}` for computed class strings.
- Design system: bg `#0f0f0f`, cards `#141414`/border `#262626`, single cyan accent `#1ea6d5`, green `#10b981` only for the `$` prompt. Terminal-session aesthetic on the home page (`$ whoami`, `$ cat about.md`, ...). Keep it dark, mono-accented, responsive.
- CI (`.github/workflows/ci.yml`) gates on `cargo fmt --check` and `clippy -D warnings`; run `make lint` before pushing. Pushing a tag `vX.Y.Z` builds and publishes a Docker image tagged `X.Y.Z` to GHCR.
- Deploy target is static hosting (cPanel/Apache): upload `dist/`; `.htaccess` handles SPA fallback. The Docker image is an alternative: a `static-web-server` (tiny scratch-based Rust static server, final image ~13 MB) serves `dist/` over plain HTTP on port 80, behind an external reverse proxy (HAProxy). SPA fallback, compression, cache headers, and CORS are all set via `SERVER_*` env vars in the Dockerfile — notably `SERVER_CORS_ALLOW_ORIGINS` (comma-separated origin list, defaults to `*`). `docker-compose.yml` runs it as a backend on a shared external `web` network behind that proxy; `SITE_URL` (build-time, single canonical domain) and `CORS_ORIGINS` are configurable. `SITE_URL` is single-valued because it's baked into canonical tags / sitemap.xml / feed.xml; serving on multiple domains is a CORS concern, not a build concern.
