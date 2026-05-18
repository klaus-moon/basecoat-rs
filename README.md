# basecoat-rs

basecoat UI for Rust: `rsx!` macro on rstml + WASM controllers + Leptos adapter.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/basecoat.svg)](https://crates.io/crates/basecoat)

A Rust port of [basecoat](https://github.com/hunvreus/basecoat) — a framework-agnostic,
Tailwind-based UI component library (itself a port of shadcn/ui for HTML templates).

---

## 30-second quickstart

```toml
# Cargo.toml
[dependencies]
basecoat            = "0.1"
basecoat-core       = "0.1"       # required: rsx! emits ::basecoat_core paths
basecoat-components = "0.1"       # required: rsx! emits ::basecoat_components paths
basecoat-macros-rt  = "0.1"       # required: rsx! emits ::basecoat_macros_rt paths
```

```rust
use basecoat::{rsx, ButtonVariant};

let html = rsx! {
    <Button variant=ButtonVariant::Primary>"Save"</Button>
};
println!("{}", html);
// → <button class="btn-primary">Save</button>
```

Run anywhere a Rust binary can run — no web framework required.

---

## Building the CSS

The Tailwind v4 + PostCSS pipeline lives at the workspace root. Run once before
building any example:

```sh
npm install          # installs tailwindcss, @tailwindcss/postcss, postcss-cli
npm run build:css    # produces style/dist/basecoat-rs.css
```

Or via xtask (checks that `node_modules/` exists first):

```sh
cargo xtask build-css
```

---

## Running the examples

Three examples live under `examples/`. Run them from the workspace root.

### `axum-ssr` — full browser demo (Dialog, Tabs, Toast hydrated)

```sh
# One-time setup — build CSS bundle and WASM controllers.
npm install && npm run build:css   # workspace CSS pipeline
cargo install wasm-pack            # if not already installed
cargo xtask build-wasm             # produces crates/basecoat-controllers/pkg/

# Run the server.
cargo run -p axum-ssr
# → http://localhost:3000
```

Open `http://localhost:3000` in a browser. You'll see Buttons, hydrated Tabs,
a Dialog with an "Open" trigger, and a "Show Toast" button that calls
`window.basecoat.toast(...)` from JavaScript.

### `leptos-islands` — Leptos adapter smoke check (CSR-only)

```sh
cargo run -p leptos-islands
```

This v0.1 example is a **CLI sanity check**, not a live browser app — it
imports every `basecoat::leptos::*` component and prints their resolved type
names to prove the feature gate works. For a real browser app, follow
`examples/leptos-islands/README.md` and the [Leptos SSR
guide](https://book.leptos.dev/) — set `features = ["hydrate"]` on
`basecoat` and use `cargo-leptos` for the two-target build.

### `static-site` — zero-framework HTML generation

```sh
cargo run -p static-site
# → writes examples/static-site/dist/index.html
```

The simplest possible use of `rsx!` — render to a string, write to disk.
Useful as a template for static site generators or build scripts.

---

## Feature matrix

| Component | Static HTML | WASM hydration | Leptos adapter |
|-----------|-------------|----------------|----------------|
| Button | v0.1 | — | v0.1 |
| Badge | v0.1 | — | v0.1 |
| Alert | v0.1 | — | v0.1 |
| Card | v0.1 | — | v0.1 |
| Input | v0.1 | — | v0.1 |
| Label | v0.1 | — | v0.1 |
| Separator | v0.1 | — | v0.1 |
| Textarea | v0.1 | — | v0.1 |
| Tooltip | v0.1 | — | v0.1 |
| Dialog | v0.1 | v0.1 | v0.1 |
| Tabs | v0.1 | v0.1 | v0.1 |
| Toast / Toaster | v0.1 | v0.1 | v0.1 |
| Dropdown | v0.2 | v0.2 | v0.2 |
| Popover | v0.2 | v0.2 | v0.2 |
| Select | v0.2 | v0.2 | v0.2 |
| Sidebar | v0.2 | v0.2 | v0.2 |
| Combobox | v0.2 | v0.2 | v0.2 |

---

## Architecture

```
basecoat-core          prop types · AttrMap · Markup · Children · classes::*
    │
    ├── basecoat-components        fn button(props) -> Markup   (string emitter)
    │       │
    │       └── class_safelist()   → Tailwind safelist
    │
    ├── basecoat-macros            rsx! proc-macro (rstml DSL)
    │       │
    │       └── basecoat-macros-rt   escape_text / escape_attr (runtime)
    │
    └── basecoat-leptos            #[component] Button / Badge / … (Leptos adapter)

basecoat-controllers   (independent WASM cdylib — Dialog / Tabs / Toast)
    └── pkg/
        ├── basecoat_controllers.js
        └── basecoat_controllers_bg.wasm
```

---

## Why basecoat-rs?

Three design bets differentiate this library from ad-hoc component modules:

1. **Own DSL on rstml.** The `rsx!` macro gives a JSX-like authoring experience
   with compile-time HTML escaping and typed prop builders — without pulling in a
   full framework for server-side rendering.

2. **WASM controllers.** The three interactive components (Dialog, Tabs, Toast)
   ship a single WASM bundle that auto-bootstraps. No framework-specific JS, no
   Alpine, no htmx dependency — just one `<script type="module">` tag.

3. **Shared class-string truth.** `basecoat_core::classes::*` is the single
   source of truth for every CSS class. Both the string emitter
   (`basecoat-components`) and the Leptos adapter (`basecoat-leptos`) call the
   same functions — class parity is structural, not tested by convention.

---

## Links

- [INTEGRATION.md](./INTEGRATION.md) — framework integration contract
- [SYNTAX.md](./crates/basecoat-macros/SYNTAX.md) — rsx! macro language spec
- [ROADMAP.md](./ROADMAP.md) — v0.2 scope and the infrastructure it needs
- [CHANGELOG.md](./CHANGELOG.md) — release notes
- [LICENSE](./LICENSE) — MIT

---

## Development & AI disclosure

Substantial portions of basecoat-rs — including the `rsx!` proc-macro, the
component implementations, and the WASM controllers — were authored with the
help of large language models (Anthropic Claude). Every commit was reviewed
and tested by a human before landing on `main`. Bug reports and PRs are
welcome regardless of how a given line was originally drafted.

---

## Acknowledgements

Based on [basecoat](https://github.com/hunvreus/basecoat) by Ronan Berder, MIT
licensed, used with permission via license. Tailwind CSS classes are reproduced
from the upstream basecoat stylesheet under the same MIT license.
