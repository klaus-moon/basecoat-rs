# leptos-fullstack

A real Leptos SSR + hydration full-stack example on top of `basecoat-rs`.

The server renders HTML with Basecoat components (Button, Tabs, Dialog, Toaster)
and sends it to the browser. The browser then hydrates the Leptos reactive tree
and the `basecoat-controllers` WASM bundle attaches interactive behaviour to
Tabs and Dialog elements.

## Prerequisites

```sh
# 1. Install cargo-leptos (the build orchestrator)
cargo install cargo-leptos

# 2. Ensure the WASM target is present (already in rust-toolchain.toml)
rustup target add wasm32-unknown-unknown

# 3. Install wasm-pack (used by the xtask build step)
cargo install wasm-pack
```

## Setup

Build the WASM controllers bundle once from the workspace root:

```sh
cargo xtask build-wasm
```

This produces `crates/basecoat-controllers/pkg/basecoat_controllers.js` (and
`.wasm`). The server serves these files at `/static/*`.

## Run

```sh
cd examples/leptos-fullstack

# Development mode — auto-reloads on source changes:
cargo leptos watch

# Production-style build and serve:
cargo leptos serve
```

Visit **http://127.0.0.1:3001** in your browser.

## What you will see

| Section | What it demonstrates |
|---------|----------------------|
| Buttons (reactive) | Leptos signal-driven counter; click updates the button label and badge |
| Tabs | Server-rendered tab markup hydrated by `basecoat-controllers` |
| Dialog | `<dialog>` element wired by the WASM controller via `data-basecoat-hydrate="dialog"` |
| Toast | JavaScript API `window.basecoat.toast(...)` through the Toaster mount point |

## Troubleshooting

**Port conflict** — change `site-addr` and `reload-port` in `Cargo.toml`
under `[package.metadata.leptos]`.

**`cargo-leptos` version** — this example was written against `cargo-leptos`
`0.2.x`. Pin with `cargo install cargo-leptos --version 0.2`.

**WASM controllers not loading** — run `cargo xtask build-wasm` first.
The bundle must exist at `crates/basecoat-controllers/pkg/` before the
server starts.

**Plain `cargo check`** — the crate compiles without cargo-leptos:

```sh
# Default (no features) — passes without cargo-leptos
cargo check -p leptos-fullstack

# SSR binary
cargo check -p leptos-fullstack --features ssr --no-default-features

# Hydrate WASM library
cargo check -p leptos-fullstack --features hydrate --no-default-features \
    --target wasm32-unknown-unknown
```
