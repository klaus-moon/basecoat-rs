# Integration Guide

Integration contract for basecoat v0.1 on Rust. This is the stable API surface
that downstream frameworks depend on.

---

## What you need to know

- `rsx!` emits absolute crate paths (`::basecoat_core`, `::basecoat_components`,
  `::basecoat_macros_rt`). Add all three as **direct** Cargo dependencies alongside
  `basecoat` — they are re-exported but must be in the extern prelude.
- The `Markup` type wraps a pre-escaped HTML string. Call `.to_string()` to get the
  `String` you put in your HTTP response.
- The WASM bundle (`basecoat_controllers.js`) is loaded once per page. It
  auto-bootstraps; call `window.basecoat.hydrate()` after dynamic content swaps.

---

## Add the dependency

```toml
[dependencies]
basecoat            = "0.1"
basecoat-core       = "0.1"
basecoat-components = "0.1"
basecoat-macros-rt  = "0.1"
```

---

## Render server-side

```rust
use basecoat::{rsx, ButtonVariant};

let html: basecoat::Markup = rsx! {
    <Button variant=ButtonVariant::Primary>"Save"</Button>
};
let html_string: String = html.to_string();
```

### Axum

```rust
use axum::response::Html;

async fn handler() -> Html<String> {
    let markup = rsx! { <Button>"Click"</Button> };
    Html(format!("<!DOCTYPE html><html><body>{markup}</body></html>"))
}
```

### Actix-web

```rust
use actix_web::HttpResponse;

async fn handler() -> HttpResponse {
    let markup = rsx! { <Button>"Click"</Button> };
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<!DOCTYPE html><html><body>{markup}</body></html>"))
}
```

### Rocket

```rust
use rocket::response::content::RawHtml;

#[rocket::get("/")]
fn index() -> RawHtml<String> {
    let markup = rsx! { <Button>"Click"</Button> };
    RawHtml(format!("<!DOCTYPE html><html><body>{markup}</body></html>"))
}
```

### Custom SSR framework with islands

```rust
// Build component outside rsx! when props need Cow values.
use basecoat::components::dialog;
use basecoat::{Children, DialogProps};
use std::borrow::Cow;

let html = dialog(DialogProps::builder()
    .id(Cow::Borrowed("my-dialog"))
    .title(Cow::Borrowed("Confirm"))
    .children(Children::from("…".to_owned()))
    .build());

// Then embed in a response body.
format!("…{html}…")
```

---

## Add the JS controllers

Include this tag in your HTML `<head>` (after building the WASM bundle):

```html
<script type="module" src="/static/basecoat_controllers.js"></script>
```

The bundle auto-bootstraps on import. For AJAX-driven page updates, call:

```js
window.basecoat.hydrate();   // re-scan DOM and attach controllers
```

### Version pinning

The JS package is versioned alongside the Rust crate. Pin to a specific version:

```html
<script type="module" src="/static/basecoat_controllers.js?v=0.1.0"></script>
```

### Build the bundle

```sh
# One-time setup — requires wasm-pack:
cargo install wasm-pack
cargo xtask build-wasm
```

Output goes to `crates/basecoat-controllers/pkg/`. Serve that directory as
`/static/` in your application.

---

## Configure Tailwind

Add the basecoat safelist so Tailwind does not purge component classes.

**Recommended pattern — `build.rs` + `@source`:**

```rust
// your-app/build.rs
fn main() {
    let safelist = basecoat_components::class_safelist();
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("basecoat-safelist.txt");
    std::fs::write(&out, safelist).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
```

```css
/* tailwind.css — reference the generated file */
@source "./target/debug/build/your-app-*/out/basecoat-safelist.txt";
```

Alternatively, copy the safelist once and commit it:

```sh
cargo run -p static-site   # produces examples/static-site/dist/
# or:
cat > basecoat-classes.txt <<'EOF'
$(cargo run -q --example print-safelist 2>/dev/null)
EOF
```

---

## Tailwind & basecoat CSS

**Important:** the CDN shortcut (`<link href="https://cdn.jsdelivr.net/npm/basecoat-css/...">` +
`<script src="cdn.tailwindcss.com">`) does **not** work with basecoat-css 0.3+.
basecoat-css now ships Tailwind v4 source using `@apply`, `@layer`, and
`@custom-variant` directives. The Tailwind v3 CDN JIT silently drops these
directives — components render with no styles. See **Pitfalls** below.

### Working pattern

Install Node 20+ (one-time), then:

1. **Add the PostCSS pipeline** to your project:
   ```sh
   npm install -D tailwindcss @tailwindcss/postcss postcss postcss-cli
   ```

2. **Create `postcss.config.mjs`:**
   ```js
   import tailwindcss from '@tailwindcss/postcss';
   export default { plugins: [tailwindcss()] };
   ```

3. **Create `style/tailwind.css`:**
   ```css
   @import "tailwindcss";

   /* Scan your application's source for utility classes. */
   @source "../src";

   /* basecoat component layer — vendor from the basecoat-rs repo or npm
      (once published). For now, copy style/basecoat.css from basecoat-rs. */
   @import "./basecoat.css";
   ```

   We don't publish an npm package yet (v0.1) — for now, vendor `style/basecoat.css`
   from the basecoat-rs repo into your project. Once we publish to npm, downstream
   users will be able to `npm install basecoat-rs` for the CSS bundle alongside
   `cargo add basecoat`.

4. **Add a build script** to `package.json`:
   ```json
   {
     "scripts": {
       "build:css": "postcss style/tailwind.css -o public/styles.css"
     }
   }
   ```

5. **Run the pipeline** before `cargo build`:
   ```sh
   npm run build:css
   ```

### Cargo-leptos users

Cargo-leptos users have two options:

- **npm pattern (recommended, consistent with other examples):** Follow the
  steps above. Remove `tailwind-input-file` from `[package.metadata.leptos]`
  and reference the pre-built CSS via your `style-file`.
- **cargo-leptos native:** Keep `tailwind-input-file = "style/tailwind.css"` in
  `[package.metadata.leptos]`. cargo-leptos downloads the Tailwind v4 standalone
  binary automatically — a parallel mechanism to the npm pipeline, but equally
  valid. Both approaches produce the same output.

### Safelist note

`basecoat_components::class_safelist()` is **not needed** with this pattern
because Tailwind already scans Rust source via the `@source` directive. Keep it
only for builds that cannot scan source (e.g. a pre-compiled crate with no
source available at build time).

### Pitfalls

| Symptom | Cause | Fix |
|---------|-------|-----|
| Components render with no styles | Using Tailwind v3 CDN + basecoat-css 0.3+ | Switch to the Tailwind v4 pipeline above |
| `@apply` / `@layer` in output verbatim | CDN JIT can't process v4 directives | Use the Tailwind v4 CLI (via npm or cargo-leptos) |
| Classes purged at production build | `@source` path wrong or safelist missing | Verify `@source "../src"` resolves to your Rust source tree |

---

## Compound components

| Upstream basecoat name | Rust `rsx!` tag | Leptos component | Direct function |
|------------------------|-----------------|------------------|-----------------|
| `<button>` | `<Button>` | `<Button>` | `basecoat::components::button(props)` |
| `<badge>` | `<Badge>` | `<Badge>` | `basecoat::components::badge(props)` |
| `<alert>` | `<Alert>` | `<Alert>` | `basecoat::components::alert(props)` |
| `<card>` | `<Card>` | `<Card>` | `basecoat::components::card(props)` |
| `<dialog>` | — (use builder) | `<Dialog>` | `basecoat::components::dialog(props)` |
| `<tabs>` | — (use builder) | `<Tabs>` | `basecoat::components::tabs(props)` |
| `<toast>` | — (use builder) | `<Toast>` | `basecoat::components::toast(props)` |
| `<toaster>` | — (use builder) | `<Toaster>` | `basecoat::components::toaster(props)` |
| `<tooltip>` | `<Tooltip>` | `<Tooltip>` | `basecoat::components::tooltip(props)` |
| `<input>` | `<Input>` | `<Input>` | `basecoat::components::input(props)` |
| `<label>` | `<Label>` | `<Label>` | `basecoat::components::label(props)` |
| `<separator>` | `<Separator>` | `<Separator>` | `basecoat::components::separator(props)` |
| `<textarea>` | `<Textarea>` | `<Textarea>` | `basecoat::components::textarea(props)` |

**Note on Dialog, Tabs, Toaster in `rsx!`:** props with `Cow<'static, str>` fields
require `Cow::Borrowed(...)` values, not bare `&str` string literals, because the
builder setter takes `impl Into<Cow<'static, str>>` and `&str` implements
`Into<Cow<'_, str>>` but not `Into<Cow<'static, str>>`. Use the builder API
directly (see example above) or pass `Cow::Borrowed("value")` as the expression.

---

## Using inside Leptos

```toml
[dependencies]
basecoat = { version = "0.1", features = ["ssr"] }
leptos = { version = "0.8", features = ["ssr"] }
```

```rust
use basecoat::leptos::*;
use basecoat::ButtonVariant;

view! {
    <Button variant=ButtonVariant::Primary attr:id="save-btn">
        "Save"
    </Button>
}
```

Pass-through HTML attributes use Leptos's `attr:` prefix syntax (Option B).
This works across CSR, SSR, and hydrate feature modes.

For CSR-only:

```toml
basecoat = { version = "0.1", features = ["csr"] }
```

For hydration:

```toml
basecoat = { version = "0.1", features = ["hydrate"] }
```

---

## Hydration contract

The three interactive components write `data-basecoat-*` attributes on their
root elements. The WASM controller reads these to bootstrap at runtime.

| Component | Root element | Required attributes | Optional attributes |
|-----------|-------------|---------------------|---------------------|
| Dialog | `<div class="dialog">` | `data-basecoat-hydrate="dialog"` `data-basecoat-version="0.1"` | `id` |
| Tabs | `<div class="tabs">` | `data-basecoat-hydrate="tabs"` `data-basecoat-version="0.1"` `data-tabs` | `id` |
| Toast / Toaster | `<div class="toaster">` | `data-basecoat-hydrate="toaster"` `data-basecoat-version="0.1"` | `id` |

**`data-basecoat-hydrate`**: identifies which controller to attach.  
**`data-basecoat-version`**: semver string for forward-compatibility checks.  
**`id`**: strongly recommended for Dialog and Tabs — the controller uses it for
aria-controls relationships and trigger-element wiring.

The controller ignores elements it has already processed (idempotent). Calling
`window.basecoat.hydrate()` after a DOM update is always safe.
