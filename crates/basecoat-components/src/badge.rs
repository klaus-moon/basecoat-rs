use basecoat_core::{BadgeProps, Markup, classes};

/// Renders a `<span class="badge{-variant}">` element.
///
/// HTML structure: `<span class="{classes}"{attrs}>{children}</span>`
pub fn badge(props: BadgeProps) -> Markup {
    let class = classes::badge(&props);
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(r#"<span class="{class}"{attrs}>{children}</span>"#))
}
