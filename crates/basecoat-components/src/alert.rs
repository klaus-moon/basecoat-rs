use crate::sub::SubProps;
use basecoat_core::{AlertProps, Markup, classes};

/// Renders a `<div class="alert{-variant}">` element.
///
/// Upstream alert structure:
/// ```html
/// <div class="alert">
///   <!-- optional SVG icon -->
///   <h2>Title</h2>
///   <section>Description</section>
/// </div>
/// ```
/// Children are injected verbatim (caller provides icon, title, description).
pub fn alert(props: AlertProps) -> Markup {
    let class = classes::alert(&props);
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(r#"<div class="{class}"{attrs}>{children}</div>"#))
}

/// Renders the `<h2>` title inside an alert.
pub fn alert_title(props: SubProps) -> Markup {
    let class_attr = match &props.class {
        Some(c) if !c.is_empty() => format!(r#" class="{}""#, c),
        _ => String::new(),
    };
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(r#"<h2{class_attr}{attrs}>{children}</h2>"#))
}

/// Renders the `<section>` description inside an alert.
pub fn alert_description(props: SubProps) -> Markup {
    let class_attr = match &props.class {
        Some(c) if !c.is_empty() => format!(r#" class="{}""#, c),
        _ => String::new(),
    };
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(
        r#"<section{class_attr}{attrs}>{children}</section>"#
    ))
}
