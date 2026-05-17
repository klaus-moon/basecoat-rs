//! `basecoat-macros` — the `rsx!` proc-macro for basecoat-rs.
//!
//! # Usage
//!
//! ```rust,ignore
//! use basecoat_macros::rsx;
//!
//! let markup = rsx! { <Button variant="primary">"Click me"</Button> };
//! ```
//!
//! See `SYNTAX.md` in this crate for the full language specification.
//!
//! # Crate architecture
//!
//! - `parse` — rstml → internal `RsxNode` AST.
//! - `emit`  — `RsxNode` AST → `proc_macro2::TokenStream`.
//! - `props` — static tables: known typed setters per component, void element
//!   list, PascalCase → snake_case conversion.
//!
//! The emitted code calls:
//! - `::basecoat_core::*` for prop types.
//! - `::basecoat_components::*` for component functions.
//! - `::basecoat_macros_rt::escape_text` / `escape_attr` for runtime escaping.

mod emit;
mod parse;
mod props;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

/// JSX-like template macro that emits pre-rendered HTML as
/// `::basecoat_core::Markup`.
///
/// See `crates/basecoat-macros/SYNTAX.md` for the full syntax specification
/// and worked examples.
///
/// # Quick reference
///
/// - **PascalCase tags** call `::basecoat_components::the_fn(Props::builder()..build())`.
/// - **lowercase tags** emit raw HTML strings.
/// - `<>...</>` is a fragment (children concatenated, no wrapper element).
/// - `attr="literal"` — compile-time-escaped value.
/// - `attr={expr}` — runtime-escaped via `::basecoat_macros_rt`.
/// - No control-flow tags — use `{ if ... }` / `{ iter.map(...) }` inside `{}`.
///
/// # Example
///
/// ```rust,ignore
/// let name = "World";
/// let markup = rsx! {
///     <div class="greeting">
///         <span>"Hello, "</span>
///         {name}
///     </div>
/// };
/// assert!(markup.to_string().contains("Hello,"));
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let input2: TokenStream2 = input.into();
    match rsx_impl(input2) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn rsx_impl(input: TokenStream2) -> syn::Result<TokenStream2> {
    // Parse via rstml.
    let nodes = rstml::parse2(input)?;

    // Translate to our AST.
    let rsx_nodes = parse::translate_nodes(nodes)?;

    // Emit the final TokenStream.
    let output = emit::emit_root(rsx_nodes);
    Ok(output)
}
