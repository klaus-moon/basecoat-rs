# basecoat-leptos

Leptos component wrappers for [basecoat-rs](../../README.md).

Each component is a real `#[component]` that emits native Leptos view nodes —
no `inner_html`, no string smuggling. Class strings are computed by
`basecoat_core::classes::*`, the same functions used by the string-emitting
`basecoat-components` backend, so both backends always emit identical CSS
classes.

## Feature flags

| Flag | Purpose |
|------|---------|
| `csr` (default) | Client-side rendering |
| `ssr` | Server-side rendering via `RenderHtml::to_html()` |
| `hydrate` | Hydration (SSR output reconciled on client) |

Enable exactly one of `csr`, `ssr`, or `hydrate` per binary target.

```toml
# In your server binary (Cargo.toml):
basecoat-leptos = { version = "0.1", default-features = false, features = ["ssr"] }

# In your WASM binary:
basecoat-leptos = { version = "0.1", default-features = false, features = ["hydrate"] }
```

## Usage

```rust
use basecoat_leptos::{Button, Badge, Card, Dialog, Tabs, TabsList, TabsTab, TabsPanel};
use basecoat_leptos::{ButtonVariant, ButtonSize, BadgeVariant};
use leptos::prelude::*;

#[component]
fn MyPage() -> impl IntoView {
    view! {
        <Card>
            <Badge variant=BadgeVariant::Secondary>"New"</Badge>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                "Save draft"
            </Button>
        </Card>
    }
}
```

## Pass-through attributes (Option B)

Leptos 0.8's `{..attrs}` spread requires `AnyAttribute` bounds which add
friction when prop types are custom enums. This crate uses **Option B**:
callers attach extra HTML attributes using Leptos's built-in `attr:` syntax
directly on the component tag.

```rust
view! {
    // id, aria-label, data-testid — all via Leptos attr: prefix
    <Button
        variant=ButtonVariant::Outline
        attr:id="save-btn"
        attr:aria-label="Save the document"
        attr:data-testid="save-button"
    >
        "Save"
    </Button>
}
```

This is idiomatic Leptos and works identically across `csr`, `ssr`, and
`hydrate` features. There is no special API needed in this crate.

## Hydration contract (Dialog, Tabs, Toast)

The interactive components emit `data-basecoat-hydrate` and
`data-basecoat-version` attributes on their root elements:

```html
<dialog
  class="dialog"
  id="my-dialog"
  data-basecoat-hydrate="dialog"
  data-basecoat-version="0.1"
>
  ...
</dialog>
```

The `basecoat-controllers` WASM bundle scans for `[data-basecoat-hydrate]`
on load and attaches the appropriate controller. The version attribute lets
the controller refuse stale SSR output after a bundle upgrade.

## Components

| Component | Element | Class |
|-----------|---------|-------|
| `Button` | `<button>` | `btn-{size?}-{variant}` |
| `Badge` | `<span>` | `badge[-variant]` |
| `Alert` | `<div role="alert">` | `alert[-destructive]` |
| `Card` | `<div>` | `card` |
| `Input` | `<input>` | `input` |
| `Label` | `<label>` | `label` |
| `Textarea` | `<textarea>` | `textarea` |
| `Separator` | `<hr role="separator">` | _(none)_ |
| `Dialog` | `<dialog>` | `dialog` |
| `DialogTrigger` | `<button>` | `btn-outline` |
| `DialogContent` | `<dialog>` | `dialog` |
| `DialogHeader` | `<header>` | `dialog-header` |
| `DialogFooter` | `<footer>` | `dialog-footer` |
| `Tabs` | `<div>` | `tabs` |
| `TabsList` | `<nav role="tablist">` | `tabs-list` |
| `TabsTab` | `<button role="tab">` | `tabs-tab` |
| `TabsPanel` | `<div role="tabpanel">` | `tabs-panel` |
| `Toaster` | `<div aria-live="polite">` | `toaster` |
| `Toast` | `<div role="status">` | `toast` |
| `Tooltip` | `<span data-tooltip>` | _(extra class only)_ |
