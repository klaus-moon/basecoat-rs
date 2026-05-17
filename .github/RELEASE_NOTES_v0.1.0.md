# basecoat-rs v0.1.0 — initial release

basecoat UI for Rust: an `rsx!` macro on rstml + WASM controllers + Leptos
adapter.

A Rust port of [basecoat](https://github.com/hunvreus/basecoat) — itself a
framework-agnostic, Tailwind-based UI component library inspired by shadcn/ui.

## Install

```toml
[dependencies]
basecoat             = "0.1"
basecoat-core        = "0.1"   # required: rsx! emits ::basecoat_core paths
basecoat-components  = "0.1"   # required: rsx! emits ::basecoat_components paths
basecoat-macros-rt   = "0.1"   # required: rsx! emits ::basecoat_macros_rt paths
```

```rust
use basecoat::{rsx, ButtonVariant};

let html = rsx! {
    <Button variant=ButtonVariant::Primary>"Save"</Button>
};
```

## Highlights

- **13 components** with a single source of truth for CSS classes
  (`basecoat_core::classes::*`): Button, Badge, Alert, Card, Input, Label,
  Separator, Textarea, Tooltip, Dialog, Tabs, Toast, Toaster.
- **Single-tag WASM hydration** for Dialog, Tabs, and Toast — no Alpine, no
  htmx. One `<script type="module">` and a `window.basecoat.hydrate()` hook.
- **Leptos adapter** with CSR/SSR/hydrate feature gates and `attr:`-prefix
  pass-through for HTML attributes.
- **Tailwind v4 + PostCSS** pipeline included; `class_safelist()` for
  builds that cannot scan source.

## What's next (v0.2)

Dropdown, Popover, Select, Sidebar, Combobox.

## Disclosure

Substantial portions of this codebase were authored with the help of large
language models (Anthropic Claude). Every commit was reviewed and tested by a
human before landing on `main`. See the README for details.

## Links

- Crate: https://crates.io/crates/basecoat
- Docs: https://docs.rs/basecoat
- Integration guide: [INTEGRATION.md](https://github.com/klaus-moon/basecoat-rs/blob/v0.1.0/INTEGRATION.md)
- License: MIT
