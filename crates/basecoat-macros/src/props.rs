//! Static tables of known component prop field names and related helpers.
//!
//! The macro must route attribute names to either a typed builder setter
//! (for known fields like `variant`, `class`, `size`, ...) or to the `AttrMap`
//! (for unknown / data-* / aria-* attrs).  Since we cannot depend on
//! `basecoat-core` at proc-macro expansion time, we hard-code the table here.
//!
//! **Must be kept in sync with `crates/basecoat-core/src/props/`.**

/// Void HTML elements that must not have children.
pub const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// Convert a PascalCase component name to the snake_case function name used in
/// `::basecoat_components`.
///
/// Algorithm:
/// 1. Lowercase the first character.
/// 2. For each subsequent uppercase ASCII letter, prepend `_` and lowercase.
///
/// ```text
/// "Button"              → "button"
/// "DialogContent"       → "dialog_content"
/// "TabsList"            → "tabs_list"
/// "AlertDialogOverlay"  → "alert_dialog_overlay"
/// ```
pub fn component_fn_name(pascal: &str) -> String {
    let mut out = String::with_capacity(pascal.len() + 4);
    for (i, c) in pascal.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// Return the list of field names that should be routed to **typed setters**
/// on the builder for the given component.
///
/// Any attribute key NOT in this list (and not `"children"`) is routed to
/// `AttrMap` via `__attrs.push(key, value)`.
///
/// The special field `"attrs"` is never a valid attribute key (it's the
/// catch-all AttrMap field itself).  The field `"children"` is populated from
/// the tag body, never from attributes.
pub fn known_typed_setters(component: &str) -> &'static [&'static str] {
    match component {
        "Button" => &["variant", "size", "class"],
        "Badge" => &["variant", "class"],
        "Alert" => &["variant", "class"],
        "Input" => &["class"],
        "Label" => &["class", "for"],
        "Textarea" => &["class"],
        "Card" => &["class"],
        "Separator" => &["orientation", "class"],
        "Dialog" => &[
            "id",
            "title",
            "description",
            "close_button",
            "close_on_overlay_click",
            "class",
        ],
        "Tabs" => &["id", "tabsets", "default_tab_index", "orientation", "class"],
        "Toast" => &["category", "title", "description", "class"],
        "Toaster" => &["id", "class"],
        "Tooltip" => &["content", "side", "class"],
        // Unknown component — no typed setters, everything goes to AttrMap.
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascal_to_snake() {
        assert_eq!(component_fn_name("Button"), "button");
        assert_eq!(component_fn_name("DialogContent"), "dialog_content");
        assert_eq!(component_fn_name("TabsList"), "tabs_list");
        assert_eq!(
            component_fn_name("AlertDialogOverlay"),
            "alert_dialog_overlay"
        );
        assert_eq!(component_fn_name("Toaster"), "toaster");
    }

    #[test]
    fn void_elements_list() {
        assert!(VOID_ELEMENTS.contains(&"br"));
        assert!(VOID_ELEMENTS.contains(&"img"));
        assert!(VOID_ELEMENTS.contains(&"input"));
        assert!(!VOID_ELEMENTS.contains(&"div"));
    }
}
