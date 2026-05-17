//! AST → TokenStream emitters.
//!
//! Each `emit_*` function takes an `RsxNode` (or sub-part thereof) and
//! appends token streams that push rendered HTML into a `String` named
//! `__s` that was declared by the caller.

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::LitStr;

use crate::parse::{RsxAttr, RsxAttrValue, RsxNode};
use crate::props::{VOID_ELEMENTS, component_fn_name, known_typed_setters};

// ── top-level entry ──────────────────────────────────────────────────────────

/// Emit the complete `rsx!` expansion for a list of top-level nodes.
///
/// Returns a `TokenStream` that evaluates to `::basecoat_core::Markup`.
pub fn emit_root(nodes: Vec<RsxNode>) -> TokenStream {
    if nodes.len() == 1 {
        // Single root — emit directly (no surrounding fragment).
        emit_node_as_markup(&nodes.into_iter().next().unwrap())
    } else {
        // Multiple roots — wrap in an implicit fragment.
        emit_fragment_as_markup(&nodes)
    }
}

// ── Markup-producing emitters ────────────────────────────────────────────────

/// Emit a single node as an expression that evaluates to `::basecoat_core::Markup`.
fn emit_node_as_markup(node: &RsxNode) -> TokenStream {
    match node {
        RsxNode::Component {
            name,
            attrs,
            children,
            span,
        } => emit_component(name, attrs, children, *span),
        RsxNode::Element {
            name,
            attrs,
            children,
            span,
        } => emit_element(name, attrs, children, *span),
        RsxNode::Fragment(children) => emit_fragment_as_markup(children),
        RsxNode::Text(s) => {
            let escaped = html_escape_text(s);
            quote! { ::basecoat_core::Markup::from_static(#escaped) }
        }
        RsxNode::Block(ts) => {
            // Expression block at root — raw, no automatic escaping.
            // The user is responsible for escaping user-controlled data.
            quote! {
                {
                    let __v = { #ts };
                    ::basecoat_core::Markup::from(::std::format!("{}", __v))
                }
            }
        }
    }
}

fn emit_fragment_as_markup(children: &[RsxNode]) -> TokenStream {
    let child_pushes = emit_children_into_string(children);
    quote! {
        {
            let mut __s = ::std::string::String::new();
            #child_pushes
            ::basecoat_core::Markup::from(__s)
        }
    }
}

// ── Component emitter ────────────────────────────────────────────────────────

fn emit_component(name: &str, attrs: &[RsxAttr], children: &[RsxNode], span: Span) -> TokenStream {
    // Compute the function name: PascalCase → snake_case
    let fn_name_str = component_fn_name(name);
    let fn_name = format_ident!("{}", fn_name_str, span = span);

    // Props type: e.g. ButtonProps
    let props_name = format_ident!("{}Props", name, span = span);

    // Split attrs into typed setters vs AttrMap passthrough.
    let typed_fields = known_typed_setters(name);

    let mut typed_setter_calls: Vec<TokenStream> = Vec::new();
    let mut extra_attrs: Vec<TokenStream> = Vec::new();

    for attr in attrs {
        let key = &attr.key;
        // children is never an attribute
        if key == "children" {
            continue;
        }

        if typed_fields.contains(&key.as_str()) {
            // Route to typed setter
            let setter = format_ident!("{}", key.replace('-', "_"), span = span);
            let val_ts = match &attr.value {
                RsxAttrValue::Literal(s) => {
                    let lit = LitStr::new(s, span);
                    quote! { #lit }
                }
                RsxAttrValue::Expr(e) => quote! { #e },
                RsxAttrValue::None => {
                    quote! { true }
                }
            };
            typed_setter_calls.push(quote! { .#setter(#val_ts) });
        } else {
            // Route to AttrMap
            let key_lit = LitStr::new(key, span);
            let val_ts = match &attr.value {
                RsxAttrValue::Literal(s) => {
                    let lit = LitStr::new(s, span);
                    quote! { #lit }
                }
                RsxAttrValue::Expr(e) => {
                    quote! { ::std::format!("{}", #e) }
                }
                RsxAttrValue::None => {
                    quote! { "" }
                }
            };
            extra_attrs.push(quote! { __attrs.push(#key_lit, #val_ts); });
        }
    }

    // Build children string
    let children_ts = if children.is_empty() {
        quote! { ::basecoat_core::Children::from(::std::string::String::new()) }
    } else {
        let child_pushes = emit_children_into_string(children);
        quote! {
            ::basecoat_core::Children::from({
                let mut __s = ::std::string::String::new();
                #child_pushes
                __s
            })
        }
    };

    quote_spanned! { span =>
        {
            let mut __attrs = ::basecoat_core::AttrMap::new();
            #( #extra_attrs )*
            ::basecoat_components::#fn_name(
                ::basecoat_core::#props_name::builder()
                    #( #typed_setter_calls )*
                    .attrs(__attrs)
                    .children(#children_ts)
                    .build()
            )
        }
    }
}

// ── HTML element emitter ─────────────────────────────────────────────────────

fn emit_element(name: &str, attrs: &[RsxAttr], children: &[RsxNode], span: Span) -> TokenStream {
    // Check void element constraint.
    let is_void = VOID_ELEMENTS.contains(&name);
    if is_void && !children.is_empty() {
        return syn::Error::new(
            span,
            format!("void element `<{}>` cannot have children", name),
        )
        .to_compile_error();
    }

    // Collect attribute token pushes.
    let attr_pushes: Vec<TokenStream> = attrs
        .iter()
        .map(|attr| emit_html_attr(attr, span))
        .collect();

    let open_tag = LitStr::new(&format!("<{}", name), span);
    let name_lit = name.to_string();

    if is_void {
        // Self-closing: <br/>, <img .../>
        quote_spanned! { span =>
            {
                let mut __s = ::std::string::String::new();
                __s.push_str(#open_tag);
                #( #attr_pushes )*
                __s.push_str("/>");
                ::basecoat_core::Markup::from(__s)
            }
        }
    } else {
        let close_tag = LitStr::new(&format!("</{}>", name_lit), span);
        let child_pushes = emit_children_into_string(children);
        quote_spanned! { span =>
            {
                let mut __s = ::std::string::String::new();
                __s.push_str(#open_tag);
                #( #attr_pushes )*
                __s.push('>');
                #child_pushes
                __s.push_str(#close_tag);
                ::basecoat_core::Markup::from(__s)
            }
        }
    }
}

/// Emit a single HTML attribute push into `__s`.
fn emit_html_attr(attr: &RsxAttr, span: Span) -> TokenStream {
    let key_lit = LitStr::new(&attr.key, span);
    match &attr.value {
        RsxAttrValue::Literal(s) => {
            // Escape at compile time.
            let escaped = html_escape_attr(s);
            let attr_str = format!(" {}=\"{}\"", attr.key, escaped);
            let attr_lit = LitStr::new(&attr_str, span);
            quote! { __s.push_str(#attr_lit); }
        }
        RsxAttrValue::Expr(e) => {
            // Escape at runtime.
            quote! {
                __s.push(' ');
                __s.push_str(#key_lit);
                __s.push_str("=\"");
                __s.push_str(&::basecoat_macros_rt::escape_attr(
                    &::std::format!("{}", #e)
                ));
                __s.push('"');
            }
        }
        RsxAttrValue::None => {
            // Boolean attribute — emit `disabled` (no value).
            let attr_str = format!(" {}", attr.key);
            let attr_lit = LitStr::new(&attr_str, span);
            quote! { __s.push_str(#attr_lit); }
        }
    }
}

// ── Children emitter ─────────────────────────────────────────────────────────

/// Emit a sequence of statements that push each child's string into `__s`.
/// Expects `__s: String` to already be declared in scope.
pub fn emit_children_into_string(children: &[RsxNode]) -> TokenStream {
    let mut stmts: Vec<TokenStream> = Vec::new();
    for child in children {
        stmts.push(emit_child_push(child));
    }
    quote! { #( #stmts )* }
}

/// Emit a single statement `__s.push_str(...)` for one child.
fn emit_child_push(node: &RsxNode) -> TokenStream {
    match node {
        RsxNode::Text(s) => {
            let escaped = html_escape_text(s);
            quote! { __s.push_str(#escaped); }
        }
        RsxNode::Block(ts) => {
            // Expression blocks are emitted raw (no automatic escaping).
            // The caller is responsible for escaping user data.  Nested
            // `rsx!{...}` calls return `Markup` whose `Display` impl emits
            // the already-safe HTML string, so they must not be double-escaped.
            // For user-controlled strings, callers should call
            // `::basecoat_macros_rt::escape_text(s)` explicitly.
            quote! {
                {
                    let __v = { #ts };
                    __s.push_str(&::std::format!("{}", __v));
                }
            }
        }
        RsxNode::Fragment(children) => {
            let inner = emit_children_into_string(children);
            quote! { #inner }
        }
        RsxNode::Component {
            name,
            attrs,
            children,
            span,
        } => {
            let markup_ts = emit_component(name, attrs, children, *span);
            quote! {
                {
                    let __markup = #markup_ts;
                    __s.push_str(&__markup.0);
                }
            }
        }
        RsxNode::Element {
            name,
            attrs,
            children,
            span,
        } => {
            let markup_ts = emit_element(name, attrs, children, *span);
            quote! {
                {
                    let __markup = #markup_ts;
                    __s.push_str(&__markup.0);
                }
            }
        }
    }
}

// ── Compile-time HTML escaping helpers ───────────────────────────────────────

/// HTML-escape a string literal value for text content at compile time.
/// Returns a `&'static str` literal token.
fn html_escape_text(s: &str) -> proc_macro2::TokenStream {
    let escaped = escape_text_str(s);
    let lit = LitStr::new(&escaped, Span::call_site());
    quote! { #lit }
}

/// HTML-escape a string for use as an attribute value at compile time.
/// Returns a plain `String` (already escaped).
fn html_escape_attr(s: &str) -> String {
    escape_attr_str(s)
}

fn escape_text_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            other => out.push(other),
        }
    }
    out
}

fn escape_attr_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}
