# basecoat-rs v0.2.0 — five new components + CSS as a crate

This release ships the five components that v0.1 deferred (Dropdown,
Popover, Select, Sidebar, Combobox), the shared controller infrastructure
they all needed, and a new `basecoat-css` crate so consumers no longer
have to vendor `style/basecoat.css`.

## Install

```toml
[dependencies]
basecoat             = "0.2"
basecoat-core        = "0.2"   # required: rsx! emits ::basecoat_core paths
basecoat-components  = "0.2"   # required: rsx! emits ::basecoat_components paths
basecoat-macros-rt   = "0.2"   # required: rsx! emits ::basecoat_macros_rt paths
basecoat-css         = "0.2"   # NEW — component-layer CSS source
```

```rust
use basecoat::{rsx, ButtonVariant};
use basecoat::leptos::{Dropdown, DropdownTrigger, DropdownMenu, DropdownItem};

let html = rsx! {
    <Button variant=ButtonVariant::Primary>"Save"</Button>
};
```

## New components

- **Dropdown** — anchored menu, floating-positioned, roving-tabindex with
  type-ahead, click-outside / Escape dismiss.
- **Popover** — anchored content panel; twelve placement variants.
- **Select** — hidden native `<select>` (form-integrated) + styled
  trigger + floating listbox + search-by-typing.
- **Sidebar** — responsive (in-flow ≥768px, overlay drawer below) with
  `localStorage` persistence per id.
- **Combobox** — input + filtered listbox with
  `aria-activedescendant` keyboard navigation.

## CSS distribution

- New crate **`basecoat-css`**: `basecoat::css::SOURCE: &'static str` and
  `basecoat_css::write_to(path)` — drop into your Tailwind v4 pipeline.
- New npm package: **`basecoat-rs-css`** — `npm install basecoat-rs-css`,
  then `@import "basecoat-rs-css/basecoat.css"`.
- No more "vendor the file from the repo" instruction in INTEGRATION.md.

## Hydration contract

WASM hydration now accepts `data-basecoat-version="0.1"` and `"0.2"`.
Existing v0.1 markup continues to hydrate against the v0.2 bundle.

## What's next (v0.3 candidates)

Accordion / Collapsible, Command palette (cmdk-style), a Rust port of the
floating-positioning engine. See [ROADMAP.md](./ROADMAP.md).

## Disclosure

Substantial portions of this codebase were developed with the help of
large language models (Anthropic Claude). Every commit was reviewed and
tested by a human before landing on `main`. See the README for details.

## Links

- Crate: https://crates.io/crates/basecoat
- CSS crate: https://crates.io/crates/basecoat-css
- Docs: https://docs.rs/basecoat
- Integration guide: [INTEGRATION.md](https://github.com/klaus-moon/basecoat-rs/blob/v0.2.0/INTEGRATION.md)
- License: MIT
