# leptos-islands example

Demonstrates `basecoat_rs::leptos::*` components (CSR-only path for v0.1).

## Why CSR-only?

Full Leptos SSR + hydration requires a two-target build pipeline orchestrated by
[`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos):

- A server binary compiled for the host target (x86_64 or aarch64)
- A WASM client compiled for `wasm32-unknown-unknown`
- A split between `#[server]` and `#[island]` functions

That setup is out of scope for a v0.1 example. CSR-only works with a plain
`cargo run` and is sufficient to verify that the Leptos component wrappers
compile and that the feature gate wiring is correct.

## Run

```sh
cargo run -p leptos-islands
```

## Upgrading to full SSR

When you're ready for SSR + hydration:

```toml
[dependencies]
basecoat-rs = { version = "0.1", features = ["hydrate"] }
leptos = { version = "0.8", features = ["hydrate"] }
```

Add `cargo-leptos` and follow the [Leptos SSR guide](https://book.leptos.dev/).
The `basecoat_rs::leptos::*` components work unchanged in all three Leptos modes
(`csr`, `ssr`, `hydrate`).

## Usage

```rust
use basecoat_rs::leptos::*;
use basecoat_rs::ButtonVariant;

view! {
    <Button variant=ButtonVariant::Primary attr:id="save-btn">
        "Save"
    </Button>
    <Badge>"New"</Badge>
    <Alert>"Something happened."</Alert>
}
```

Pass-through HTML attributes use Leptos's `attr:` prefix (Option B). This avoids
the `AnyAttribute` friction of attribute spreading and works in all three modes.
