# axum-ssr example

Axum SSR server that renders a page via `rsx!` and serves the WASM controllers
bundle so Dialog, Tabs, and Toast hydrate in the browser.

CSS is compiled at the workspace root using the Tailwind v4 PostCSS pipeline
(`npm run build:css`) and embedded into the binary via `include_bytes!`.

## Prerequisites

### 1. Build the CSS bundle (workspace root, one-time)

```sh
npm install
npm run build:css
```

This produces `style/dist/basecoat-rs.css` at the workspace root. The binary
embeds this file via `include_bytes!` — if it does not exist, `cargo build`
will fail with a clear error pointing to the missing path.

### 2. Build the WASM controllers bundle (one-time)

```sh
cargo install wasm-pack   # if not already installed
cargo xtask build-wasm
```

## Run

```sh
cargo run -p axum-ssr
# Open http://localhost:3000
```

If port 3000 is taken:

```sh
PORT=4000 cargo run -p axum-ssr
# Open http://localhost:4000
```

## How tree-shaking works

`style/tailwind.css` at the workspace root uses Tailwind v4 `@source` directives
to scan Rust source files for utility class names:

```css
@source "../crates/basecoat-core/src";
@source "../crates/basecoat-components/src";
@source "../crates/basecoat-leptos/src";
@source "../crates/basecoat/src";
@source "../examples";
```

Tailwind only emits CSS for classes it finds in those files — unused utilities
are dropped automatically. No safelist configuration is required.

## Tailwind safelist API (no-build-step setups)

The `class_safelist()` API from `basecoat-components` is still useful when you
cannot run a local Tailwind build (e.g. static-site generation with a CDN
fallback). See `examples/static-site/` for that pattern.

## Compiled CSS location

After running `npm run build:css` at the workspace root:

```
style/dist/basecoat-rs.css    # tree-shaken CSS bundle
```

The CSS is embedded into the binary via `include_bytes!` at compile time and
served at `/static/styles.css` from memory — no disk I/O at request time.
