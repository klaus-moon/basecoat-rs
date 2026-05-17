//! `basecoat` — umbrella re-export crate.
//!
//! This is the single entry point for consuming basecoat UI components in Rust.
//! Add `basecoat` to your `Cargo.toml` and import everything from here.
//!
//! # 30-second start
//!
//! ```
//! use basecoat::{rsx, ButtonVariant};
//!
//! let html: basecoat::Markup = rsx! {
//!     <Button variant={ButtonVariant::Primary}>"Click"</Button>
//! };
//! assert!(html.to_string().contains("btn-primary"));
//! assert!(html.to_string().contains("Click"));
//! ```

// ── Core types, prop structs, enums, class functions, and the derive macro ──
pub use basecoat_core::*;

// ── String-emitting component functions (fn button(props) -> Markup, etc.) ──
pub use basecoat_components as components;

// ── rsx! proc-macro ──────────────────────────────────────────────────────────
pub use basecoat_macros::rsx;

// ── Runtime helpers required for rsx! expansion ──────────────────────────────
//
// The code emitted by `rsx!` calls `::basecoat_macros_rt::escape_attr` and
// `::basecoat_macros_rt::escape_text` as absolute paths.  For those paths to
// resolve in user crates, `basecoat_macros_rt` must be linked into the
// compilation unit.  Re-exporting it here (so that `basecoat` depends on
// it) satisfies the linker, because Cargo propagates the `extern crate` for
// every transitive dependency.
pub use basecoat_macros_rt;

// ── Optional Leptos adapter ───────────────────────────────────────────────────
#[cfg(feature = "leptos")]
pub use basecoat_leptos as leptos;
