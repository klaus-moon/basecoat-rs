use basecoat_core::{ButtonProps, Markup, classes};

/// Renders a `<button>` element with basecoat button classes.
///
/// HTML structure: `<button class="{classes}">{children}</button>`
pub fn button(props: ButtonProps) -> Markup {
    let class = classes::button(&props);
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(
        r#"<button class="{class}"{attrs}>{children}</button>"#
    ))
}
