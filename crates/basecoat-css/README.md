# basecoat-css

Component-layer CSS source for [basecoat-rs](https://github.com/klaus-moon/basecoat-rs),
the Rust port of [basecoat](https://github.com/hunvreus/basecoat).

This crate ships the same `basecoat.css` that the upstream project distributes
on npm. Use it inside a Tailwind v4 pipeline via `@import`, or call
`basecoat_css::write_to(path)` from a `build.rs` to drop the file on disk.

## Quick start

```rust
// build.rs
fn main() {
    let dest = std::path::Path::new("style/basecoat.css");
    basecoat_css::write_to(dest).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
```

```css
/* style/tailwind.css */
@import "tailwindcss";
@source "../src";
@import "./basecoat.css";
```

See the main [README](https://github.com/klaus-moon/basecoat-rs#readme) and
[INTEGRATION.md](https://github.com/klaus-moon/basecoat-rs/blob/main/INTEGRATION.md)
for the full integration story.

Substantial portions of basecoat-rs were authored with the help of large
language models.
