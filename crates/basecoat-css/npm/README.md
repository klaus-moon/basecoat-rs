# basecoat-rs-css

Component-layer CSS for [basecoat-rs](https://github.com/klaus-moon/basecoat-rs),
the Rust port of [basecoat](https://github.com/hunvreus/basecoat).

This package is the npm-pipeline counterpart to the
[`basecoat-css`](https://crates.io/crates/basecoat-css) Rust crate. Pick the one
that fits your build.

## Install

```sh
npm install basecoat-rs-css
```

## Use

```css
/* style/tailwind.css */
@import "tailwindcss";
@source "../src";
@import "basecoat-rs-css/basecoat.css";
```

Then run your normal Tailwind v4 build (PostCSS, the Tailwind CLI, or
cargo-leptos's bundled Tailwind binary).

See the main
[README](https://github.com/klaus-moon/basecoat-rs#readme) and
[INTEGRATION.md](https://github.com/klaus-moon/basecoat-rs/blob/main/INTEGRATION.md)
for the full integration story.

Substantial portions of basecoat-rs were authored with the help of large
language models.
