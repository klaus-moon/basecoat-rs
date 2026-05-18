# Changelog

All notable changes to basecoat-rs are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] — 2026-05-18

Adds the five components that v0.1 deferred (Dropdown, Popover, Select,
Sidebar, Combobox) and ships the component-layer CSS as a first-class
dependency instead of asking consumers to vendor a file.

### Added
- Five new components — each with prop types, class-string fn, HTML
  emitter, Leptos adapter, and WASM controller:
  - **Dropdown** — anchored menu over a `<details>` trigger, with
    floating-element positioning, roving-tabindex (vertical, wrap,
    type-ahead), click-outside / Escape dismiss.
  - **Popover** — anchored content panel with twelve placement variants
    (`PopoverPlacement` enum); dismiss primitive.
  - **Select** — hidden native `<select>` for form integration plus a
    custom-styled trigger button and a floating listbox. Search-by-typing
    via the roving-tabindex helper.
  - **Sidebar** — responsive (in-flow above 768px, overlay drawer below)
    with `localStorage` persistence (`basecoat:sidebar:{id}` key).
  - **Combobox** — input with filtered listbox suggestions and
    `aria-activedescendant` keyboard navigation (focus stays in the input).
- **Shared WASM controller infrastructure** at
  `basecoat-controllers/src/controllers/`:
  - `keyboard::RovingTabindex` — ArrowUp/Down/Home/End/type-ahead with
    detachable Drop-based listeners.
  - `dismiss::attach` — click-outside (capture phase) + Escape + focus-out,
    detachable.
  - `floating` — Rust → JS bridge over `@floating-ui/dom` (vendored at
    `pkg/vendor/floating-ui.dom.esm.js`; see ROADMAP for the Rust-port
    decision deferred to v0.3).
  - `media` — `matchMedia` wrapper with detachable listener.
  - `util::dispatch_initialized` — extracted from dialog/tabs/toast.
- New crate **`basecoat-css`** (`0.2.0`) — ships the component-layer CSS
  via `basecoat_css::SOURCE: &'static str` and `basecoat_css::write_to(path)`.
  Re-exported as `basecoat::css`. Replaces the v0.1 "vendor `style/basecoat.css`"
  instruction in INTEGRATION.md.
- New **npm package staging** at `crates/basecoat-css/npm/` —
  `basecoat-rs-css` shim; publish via `xtask publish-npm-css`.

### Changed
- WASM hydration now accepts `data-basecoat-version="0.2"` in addition to
  `"0.1"`. Existing v0.1 markup continues to hydrate unchanged.
- `tabs.rs` controller refactored to use the new `keyboard::RovingTabindex`
  helper — behavior unchanged.
- Workspace umbrella crate `basecoat` now re-exports `basecoat_css` as
  `basecoat::css`.
- WASM size budget raised from 120KB to 200KB gzipped (current: ~30KB +
  vendored floating-ui.dom.esm.js as a sibling file).

### Decided
Open questions called out in v0.1's ROADMAP resolved as:
- Floating positioning: **Option C** — bundle `@floating-ui/dom` as a JS
  shim; revisit a Rust port in v0.3.
- Select form integration: **hidden `<select>`**, same as the upstream
  basecoat reference.
- Sidebar inclusion: **shipped in v0.2** (one milestone).

[0.2.0]: https://github.com/klaus-moon/basecoat-rs/releases/tag/v0.2.0

## [0.1.0] — 2026-05-18

Initial release.

### Added
- Umbrella crate `basecoat` re-exporting the full v0.1 API surface.
- `rsx!` proc-macro on rstml — JSX-like authoring with compile-time HTML
  escaping and typed prop builders.
- 13 components with one source of truth for CSS classes
  (`basecoat_core::classes::*`): Button, Badge, Alert, Card, Input, Label,
  Separator, Textarea, Tooltip, Dialog, Tabs, Toast, Toaster.
- `basecoat-controllers` — WASM `cdylib` providing a single auto-bootstrapping
  bundle that hydrates Dialog, Tabs, and Toast (no Alpine, no htmx).
- `basecoat-leptos` adapter with `csr`/`ssr`/`hydrate` feature gates and an
  `attr:`-prefix pass-through for HTML attributes.
- Tailwind v4 + PostCSS pipeline at the workspace root and a
  `basecoat_components::class_safelist()` API for build environments that
  cannot scan source.
- Three examples: `static-site` (zero-framework), `axum-ssr` (full browser
  demo with hydrated Dialog / Tabs / Toast), and `leptos-islands` (CSR-only
  feature-gate sanity check).

### Compatibility
- MSRV: stable Rust with edition 2024.
- Tailwind v4 (the v3 CDN does not process `@apply` / `@layer` /
  `@custom-variant` and will silently produce unstyled output).

[Unreleased]: https://github.com/klaus-moon/basecoat-rs/compare/v0.2.0...HEAD
[0.1.0]: https://github.com/klaus-moon/basecoat-rs/releases/tag/v0.1.0
