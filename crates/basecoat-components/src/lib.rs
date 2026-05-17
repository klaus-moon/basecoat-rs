//! basecoat-components — HTML-string component functions.
//!
//! Each function takes a typed `*Props` struct from `basecoat-core` and returns
//! a `Markup` value containing the rendered HTML fragment.
//!
//! # Tailwind safelist
//!
//! The `class_safelist()` function returns the contents of the generated
//! `basecoat-classes.txt` file (built by `build.rs`).  Consumers can write it
//! to disk from their own build scripts:
//!
//! ```rust,ignore
//! fn main() {
//!     let safelist = basecoat_components::class_safelist();
//!     std::fs::write("basecoat-classes.txt", safelist).unwrap();
//! }
//! ```

pub mod alert;
pub mod badge;
pub mod button;
pub mod card;
pub mod dialog;
pub mod input;
pub mod label;
pub mod separator;
pub mod sub;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod tooltip;

// ── Re-export all top-level component functions ──────────────────────────────

pub use alert::{alert, alert_description, alert_title};
pub use badge::badge;
pub use button::button;
pub use card::{card, card_content, card_description, card_footer, card_header, card_title};
pub use dialog::dialog;
pub use input::input;
pub use label::label;
pub use separator::separator;
pub use sub::SubProps;
pub use tabs::tabs;
pub use textarea::textarea;
pub use toast::{toast, toaster};
pub use tooltip::tooltip;

// ── Tailwind safelist ────────────────────────────────────────────────────────

/// Returns the static list of every Tailwind class that any component can emit.
///
/// Each class is on its own line.  Add this to your Tailwind config with:
/// ```text
/// @source "path/to/basecoat-classes.txt";
/// ```
pub fn class_safelist() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/basecoat-classes.txt"))
}
