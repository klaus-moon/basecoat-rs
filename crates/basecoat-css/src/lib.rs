//! Component-layer CSS for basecoat-rs.
//!
//! Drop into a Tailwind v4 pipeline via `@import` or write to disk and
//! reference the file from your build.
//!
//! # Examples
//!
//! ```no_run
//! use std::path::Path;
//! basecoat_css::write_to(Path::new("style/basecoat.css")).unwrap();
//! ```

/// The full basecoat-css source as a `&'static str`.
pub const SOURCE: &str = include_str!("../assets/basecoat.css");

/// Write the source to `path`, overwriting if it exists.
pub fn write_to(path: &std::path::Path) -> std::io::Result<()> {
    std::fs::write(path, SOURCE)
}
