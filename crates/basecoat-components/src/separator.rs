use basecoat_core::{Markup, SeparatorProps};

/// Renders an `<hr role="separator">` element.
///
/// Upstream basecoat uses `role="separator"` on `<hr>` — no CSS class needed.
/// Styling is via CSS attribute selectors on `[role="separator"]`.
///
/// HTML structure: `<hr role="separator"{attrs}>`
pub fn separator(props: SeparatorProps) -> Markup {
    let attrs = props.attrs.render();
    Markup::from(format!(r#"<hr role="separator"{attrs}>"#))
}
