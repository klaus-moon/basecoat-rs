use crate::sub::SubProps;
use basecoat_core::{CardProps, Markup, classes};

/// Renders a `<div class="card">` container. Children are injected verbatim.
///
/// Upstream card uses semantic HTML child elements directly (no wrapper sub-divs):
/// ```html
/// <div class="card">
///   <header><h2>Title</h2><p>Description</p></header>
///   <section>Content</section>
///   <footer>Footer</footer>
/// </div>
/// ```
///
/// Compound sub-component helpers: `card_header`, `card_content`, `card_footer`.
pub fn card(props: CardProps) -> Markup {
    let class = classes::card(&props);
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(r#"<div class="{class}"{attrs}>{children}</div>"#))
}

/// Renders a `<header>` inside a card.
/// Upstream structure: `<header><h2>{title}</h2><p>{description}</p></header>`
/// Use `children` for arbitrary content.
pub fn card_header(props: SubProps) -> Markup {
    let class_attr = match &props.class {
        Some(c) if !c.is_empty() => format!(r#" class="{}""#, c),
        _ => String::new(),
    };
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(r#"<header{class_attr}{attrs}>{children}</header>"#))
}

/// Renders a `<section>` inside a card (the body content area).
pub fn card_content(props: SubProps) -> Markup {
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

/// Renders a `<footer>` inside a card.
pub fn card_footer(props: SubProps) -> Markup {
    let class_attr = match &props.class {
        Some(c) if !c.is_empty() => format!(r#" class="{}""#, c),
        _ => String::new(),
    };
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(r#"<footer{class_attr}{attrs}>{children}</footer>"#))
}

/// Renders an `<h2>` title inside a card header.
pub fn card_title(props: SubProps) -> Markup {
    let class_attr = match &props.class {
        Some(c) if !c.is_empty() => format!(r#" class="{}""#, c),
        _ => String::new(),
    };
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(r#"<h2{class_attr}{attrs}>{children}</h2>"#))
}

/// Renders a `<p>` description inside a card header.
pub fn card_description(props: SubProps) -> Markup {
    let class_attr = match &props.class {
        Some(c) if !c.is_empty() => format!(r#" class="{}""#, c),
        _ => String::new(),
    };
    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(r#"<p{class_attr}{attrs}>{children}</p>"#))
}
