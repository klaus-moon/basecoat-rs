# axum-ssr example

Axum SSR server that renders a page via `rsx!` and serves the WASM controllers
bundle so Dialog, Tabs, and Toast hydrate in the browser.

## Setup (one-time)

Build the WASM bundle:

```sh
cargo install wasm-pack          # if not already installed
cargo xtask build-wasm
```

## Run

```sh
cargo run -p axum-ssr
# Open http://localhost:3000
```

## Tailwind safelist wiring

To use Tailwind CSS with tree-shaking in your own project, the basecoat class
safelist must be visible to Tailwind at build time. The recommended pattern:

**1. Write the safelist from `build.rs`:**

```rust
// build.rs
fn main() {
    let safelist = basecoat_components::class_safelist();
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("basecoat-safelist.txt");
    std::fs::write(&out, safelist).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
```

**2. Reference it from your Tailwind config:**

```css
/* tailwind.css */
@source "./target/debug/build/<your-crate>-*/out/basecoat-safelist.txt";
```

Or use a stable path by copying the file as a post-build step:

```sh
# In your CI or Makefile:
cargo build && \
  find target -name basecoat-safelist.txt | head -1 | xargs -I{} cp {} basecoat-classes.txt
```

```css
@source "./basecoat-classes.txt";
```

This example uses the CDN version of Tailwind for simplicity. In production,
use a local Tailwind build with the safelist wired in as shown above.
