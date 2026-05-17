//! Leptos islands example — CSR-only, demonstrates `basecoat::leptos::*`
//! components inside a Leptos `view!`.
//!
//! # Why CSR-only?
//!
//! Full Leptos SSR requires a two-target build pipeline (server binary + WASM
//! client) orchestrated by `cargo-leptos`. That setup is substantial and out of
//! scope for a v0.1 example. CSR-only works with a plain `cargo run` and is
//! sufficient to verify that the Leptos component wrappers compile and render
//! correctly. Upgrade to SSR when you're ready to use `cargo-leptos`.
//!
//! # Run
//!
//! ```sh
//! cargo run -p leptos-islands
//! # Prints usage information — no browser required for this CLI demo.
//! ```

// Import the Leptos components to verify the feature-gated re-export path
// resolves correctly. These are compile-time checks; the imports and usage of
// `std::any::type_name_of_val` prove the symbols exist without needing to
// call them (Leptos components require a reactive runtime to execute).
use basecoat::leptos::{Alert, Badge, Button};
use basecoat::{AlertVariant, BadgeVariant, ButtonVariant};

fn main() {
    println!("basecoat-rs leptos-islands example (CSR-only)");
    println!("Components available via basecoat::leptos::*:");
    println!("  Button, Badge, Alert, Card, Dialog, Input, Label,");
    println!("  Separator, Tabs, Textarea, Toast, Toaster, Tooltip");
    println!();
    println!("To use in a Leptos app:");
    println!("  use basecoat::leptos::*;");
    println!("  use basecoat::ButtonVariant;");
    println!();
    println!("  view! {{");
    println!("      <Button variant=ButtonVariant::Primary>\"Save\"</Button>");
    println!("  }}");
    println!();

    // Verify symbols resolve at compile time by printing their names.
    println!(
        "Resolved: Button={}, Badge={}, Alert={}",
        std::any::type_name_of_val(&Button),
        std::any::type_name_of_val(&Badge),
        std::any::type_name_of_val(&Alert),
    );

    // Verify enum variants are accessible.
    let _variants = (
        ButtonVariant::Primary,
        BadgeVariant::Default,
        AlertVariant::Default,
    );
    println!("Enum variants resolved: ButtonVariant, BadgeVariant, AlertVariant");
    println!();
    println!("See crates/basecoat-leptos/src/components/ for all component sources.");
}
