//! Leptos component wrappers for basecoat-rs.
//!
//! Each component is a real `#[component]` that emits native Leptos nodes
//! (not `inner_html`). Class strings are shared with `basecoat_core::classes::*`
//! so both this crate and the string-emitting `basecoat-components` crate always
//! produce identical CSS classes.
//!
//! ## Feature flags
//!
//! - `csr` (default) — client-side rendering
//! - `ssr` — server-side rendering / `RenderHtml::to_html()`
//! - `hydrate` — hydration (SSR + CSR reconciliation)
//!
//! ## Pass-through attributes (Option B)
//!
//! Leptos 0.8 attribute spreading via `{..attrs}` requires `AnyAttribute` which
//! adds friction for custom enum prop types. We use **Option B**: callers add
//! extra HTML attributes using Leptos's built-in `attr:` syntax directly on the
//! component invocation:
//!
//! ```rust,ignore
//! use basecoat_leptos::Button;
//! use basecoat_core::ButtonVariant;
//!
//! view! {
//!     <Button variant=ButtonVariant::Outline attr:id="my-btn" attr:aria-label="Save">
//!         "Save"
//!     </Button>
//! }
//! ```
//!
//! This is idiomatic Leptos and works across all three feature modes.

pub mod components;

pub use components::*;

// Re-export the enums and types that callers need to pass as props,
// so consumers only need to depend on `basecoat-leptos`.
pub use basecoat_core::props::tooltip::TooltipSide;
pub use basecoat_core::{
    AlertVariant, BadgeVariant, ButtonSize, ButtonVariant, TabsOrientation, ToastCategory,
};
