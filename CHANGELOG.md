# Changelog

All notable changes to basecoat-rs are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/klaus-moon/basecoat-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/klaus-moon/basecoat-rs/releases/tag/v0.1.0
