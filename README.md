# Blog

A **personal blog** focused on **software development**, built fully in **Rust** with [Yew](https://yew.rs/) (WebAssembly) and Markdown content.

This blog serves as a space to share knowledge, insights, and experiences in software development, programming, and web technologies.

## Project Structure

```
blog/
├── content/
│   └── blog/          # Blog posts (MDX/Markdown files)
├── src/
│   ├── main.rs        # App entry + router
│   ├── components/    # Yew components (header, footer, layout)
│   ├── pages/         # Pages (home, blog list, blog post, 404)
│   ├── posts.rs       # Post data (embedded at compile time)
│   ├── markdown.rs    # Markdown -> HTML rendering
│   └── meta.rs        # Document title/meta helpers
├── styles/
│   └── globals.css    # Tailwind CSS v4 input + custom styles
├── static/            # Static assets copied to dist (images, .htaccess)
├── build.rs           # Parses post frontmatter, generates sitemap/robots
├── index.html         # Trunk entry point
├── Trunk.toml         # Trunk configuration
└── Cargo.toml         # Rust dependencies
```

## Blog Posts

Blog posts are stored as `.mdx` files in `content/blog/`. Each post is a markdown file with YAML frontmatter. See `content/README.md` for the format specification.

Posts are parsed at **compile time** by `build.rs` and embedded directly into the WebAssembly binary — no runtime file access or API needed.

## Getting Started

### Prerequisites

```bash
# Rust toolchain (https://rustup.rs)
rustup target add wasm32-unknown-unknown

# Trunk (WASM bundler) — https://trunkrs.dev
cargo install trunk --locked
# or download a prebuilt binary from https://github.com/trunk-rs/trunk/releases
```

### Development

```bash
# Run development server (http://localhost:3000)
trunk serve

# Build for production (outputs to dist/)
SITE_URL=https://your-domain.com trunk build --release
```

`SITE_URL` controls the URLs written into the generated `sitemap.xml`, `robots.txt`, and `feed.xml` (RSS) — defaults to `http://localhost:3000`. Prefer `make build` over plain `trunk build` so these files are regenerated before the asset pipeline copies them.

## Tech Stack

- **Rust + Yew** - Frontend framework compiled to WebAssembly
- **yew-router** - Client-side routing
- **Trunk** - Build tool and dev server
- **Tailwind CSS v4** - Utility-first CSS (compiled by Trunk)
- **pulldown-cmark** - Markdown rendering
- **highlight.js** - Syntax highlighting for code blocks

## Deployment

The production build is a fully static site in `dist/` — it can be hosted anywhere that serves static files.

### cPanel / Apache

1. **Build the project:**
   ```bash
   SITE_URL=https://your-domain.com trunk build --release
   ```

2. **Upload the contents of `dist/`** to your web root (e.g. `public_html/`).

The included `.htaccess` (copied into `dist/` automatically) handles:
- Serving `index.html` for all SPA routes (e.g. `/blog/my-post`)
- The `application/wasm` MIME type

No Node.js app, server process, or `npm install` is required — it's just static files.
