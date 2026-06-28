# CLAUDE.md

Blog written fully in Rust: Leptos (CSR, WebAssembly) compiled with Trunk. No Node.js anywhere.

## Commands

```bash
make dev                   # dev server on http://localhost:3000 (hot reload on file change)
make build                 # production build -> dist/ (uses SITE_URL env, default http://localhost:3000)
make image                 # build docker image (IMAGE/TAG/SITE_URL overridable)
make clean                 # cargo clean + remove dist/
cargo check --target wasm32-unknown-unknown   # fast compile check
# lint gate (same as CI — no make target):
cargo fmt --all -- --check
cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings
```

Toolchain: stable Rust with the `wasm32-unknown-unknown` target, plus the `trunk` binary. Trunk downloads Tailwind CSS v4 and wasm-bindgen automatically (versions pinned in `Trunk.toml` / `Cargo.lock`).

## Architecture

- **Posts are compiled in.** `build.rs` parses YAML frontmatter from `content/blog/*.mdx`, generates `$OUT_DIR/posts_gen.rs` (included by `src/posts.rs`), and writes `static/sitemap.xml`, `static/robots.txt`, and `static/feed.xml` (RSS 2.0) using the `SITE_URL` env var. Adding a post = drop an `.mdx` file in `content/blog/` and rebuild. The generated static files are gitignored — never edit them by hand, and always run the build (or `cargo check`) before `trunk build` so they're fresh (trunk copies assets in parallel with the cargo build; `make build` and the Dockerfile already do this).
- `src/main.rs` — structural router (`/`, `/blog`, `/blog/:slug`, 404 via `<Routes fallback>`) and app shell (`<Router><Layout>…</Layout></Router>`).
- `src/config.rs` — site-level constants only (`SITE_NAME`, `SITE_TAGLINE`). No personal/resume content. Edit copy here, not in `pages/home.rs`.
- `src/meta.rs` — runtime SEO: per-route title, description, canonical, Open Graph/Twitter tags, `article:*` times. Every page sets its meta in an `Effect::new` hook (pure web-sys, framework-agnostic).
- `src/markdown.rs` — pulldown-cmark → HTML `String`, injected at the call site via the Leptos `inner_html=` attribute. Element styling comes from `.markdown-body` rules in `styles/globals.css`, not per-element classes.
- Syntax highlighting: highlight.js loaded from CDN in `index.html` (SRI-pinned); `meta::highlight_code_blocks()` re-runs it after a post renders.
- `index.html` is the Trunk entry: Tailwind input, copied static assets (`robots.txt`, `sitemap.xml`, `.htaccess`, `static/images/`), fonts (Bricolage Grotesque display, JetBrains Mono).

## Conventions & gotchas

- Tailwind v4 scans `src/` and `index.html` via `@source` in `styles/globals.css` — utility classes inside `view!` string literals are picked up. Custom classes (`container-responsive`, `page-hero`, `glow-card`, `rise`) are plain CSS in `globals.css`.
- Leptos 0.8 (CSR): `#[component] fn X(...) -> impl IntoView` + the `view!` macro. Links use `<A href=…>`; active state via `use_location`, route params via `use_params_map`. Dynamic classes use a reactive closure (`attr:class=move || format!(…)`); on components, extra HTML attributes need the `attr:` prefix. Conditional markup: `cond.then(|| view!{…})` or `move || if … { a.into_any() } else { b.into_any() }`.
- Design system: bg `#0f0f0f`, cards `#141414`/border `#262626`, single cyan accent `#1ea6d5`. Clean, readable blog layout (not a terminal). Keep it dark, accent-restrained, responsive; `.rise` adds a subtle staggered page-load fade (respects `prefers-reduced-motion`).
- CI (`.github/workflows/ci.yml`) gates on `cargo fmt --check` and `clippy -D warnings`; run those two before pushing (no `make` target for them). Pushing a tag `vX.Y.Z` builds and publishes a Docker image tagged `X.Y.Z` to GHCR.
- Deploy target is static hosting (cPanel/Apache): upload `dist/`; `.htaccess` handles SPA fallback. The Docker image is an alternative (`make image`): a `static-web-server` (tiny scratch-based Rust static server, final image ~13 MB) serves `dist/` over plain HTTP on port 80, typically behind a reverse proxy (HAProxy) that terminates TLS. SPA fallback, compression, cache headers, and CORS are all set via `SERVER_*` env vars baked into the Dockerfile — notably `SERVER_CORS_ALLOW_ORIGINS` (comma-separated origin list, defaults to `*`); override any of them at run time with `docker run -e`. No docker-compose — run the image directly. `SITE_URL` (build-time, single canonical domain) is baked into canonical tags / sitemap.xml / feed.xml, so it's single-valued; serving on multiple domains is a CORS concern, not a build concern.
