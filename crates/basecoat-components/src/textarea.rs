use basecoat_core::attrs::escape_attr;
use basecoat_core::{Markup, TextareaProps, classes};

/// Renders a `<textarea>` element with basecoat textarea classes.
///
/// HTML structure: `<textarea class="{classes}" placeholder="..." {attrs}></textarea>`
pub fn textarea(props: TextareaProps) -> Markup {
    let class = classes::textarea(&props);
    let mut extra = String::new();

    if let Some(placeholder) = &props.placeholder {
        extra.push_str(&format!(r#" placeholder="{}""#, escape_attr(placeholder)));
    }
    if props.disabled {
        extra.push_str(" disabled");
    }

    let attrs = props.attrs.render();
    Markup::from(format!(
        r#"<textarea class="{class}"{extra}{attrs}></textarea>"#
    ))
}
