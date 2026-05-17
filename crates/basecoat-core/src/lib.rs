//! basecoat-core — foundational types, prop structs, and class-string functions.

pub mod attrs;
pub mod children;
pub mod classes;
pub mod markup;
pub mod props;

pub use attrs::AttrMap;
pub use children::Children;
pub use markup::Markup;
pub use props::*;

// Re-export the derive macro so consumers write `use basecoat_core::BasecoatProps`.
pub use basecoat_core_macros::BasecoatProps;
