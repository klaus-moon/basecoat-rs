//! Runtime helpers for the `rsx!` proc-macro.
//!
//! This crate is intentionally tiny — it provides HTML-escaping functions that
//! the code emitted by `basecoat-macros` calls at runtime.  It must be a
//! *separate* crate because proc-macro crates cannot export non-macro items
//! (the `proc-macro = true` restriction means `lib.rs` items other than
//! `#[proc_macro]` fns are not accessible to downstream crates).
//!
//! The umbrella `basecoat-rs` crate re-exports this as
//! `basecoat_rs::macros_rt` (or transitively, so users don't need to add it
//! to their own `Cargo.toml`).  Phase 3 must add:
//!
//! ```toml
//! # in crates/basecoat-rs/Cargo.toml
//! basecoat-macros-rt = { workspace = true }
//! ```
//!
//! and re-export it:
//!
//! ```rust
//! // in crates/basecoat-rs/src/lib.rs
//! pub use basecoat_macros_rt as macros_rt;
//! ```

use std::borrow::Cow;

/// HTML-escape a string for use as **element text content**.
///
/// Escapes `&`, `<`, `>`.  Does *not* escape `"` or `'` (not needed in text
/// nodes, only in attribute values).
///
/// Returns a `Cow::Borrowed` if no escaping is needed (fast path).
pub fn escape_text(s: &str) -> Cow<'_, str> {
    let needs_escape = s.bytes().any(|b| matches!(b, b'&' | b'<' | b'>'));
    if !needs_escape {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            other => out.push(other),
        }
    }
    Cow::Owned(out)
}

/// HTML-escape a string for use as an **attribute value** (inside `"..."`).
///
/// Escapes `&`, `<`, `>`, `"`, `'`.
///
/// Returns a `Cow::Borrowed` if no escaping is needed (fast path).
pub fn escape_attr(s: &str) -> Cow<'_, str> {
    let needs_escape = s
        .bytes()
        .any(|b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''));
    if !needs_escape {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 8);
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
    Cow::Owned(out)
}

/// Escape a `Display` value for use as element text content.
///
/// This is the function emitted by `rsx!` for `{expr}` children blocks where
/// the expression is not known to be a `Markup` value.
pub fn escape_display(val: &dyn std::fmt::Display) -> String {
    escape_text(&val.to_string()).into_owned()
}

/// Escape a `Display` value for use as an attribute value.
///
/// This is the function emitted by `rsx!` for `attr={expr}` on HTML elements.
pub fn escape_attr_display(val: &dyn std::fmt::Display) -> String {
    escape_attr(&val.to_string()).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_text_no_alloc_on_safe() {
        let s = "hello world";
        assert!(matches!(escape_text(s), Cow::Borrowed(_)));
    }

    #[test]
    fn escape_text_escapes_lt_gt_amp() {
        assert_eq!(escape_text("<b>&amp;</b>"), "&lt;b&gt;&amp;amp;&lt;/b&gt;");
    }

    #[test]
    fn escape_attr_escapes_quotes() {
        assert_eq!(
            escape_attr(r#"say "hi" & 'bye'"#),
            "say &quot;hi&quot; &amp; &#39;bye&#39;"
        );
    }

    #[test]
    fn escape_attr_no_alloc_on_safe() {
        let s = "just-a-value";
        assert!(matches!(escape_attr(s), Cow::Borrowed(_)));
    }
}
