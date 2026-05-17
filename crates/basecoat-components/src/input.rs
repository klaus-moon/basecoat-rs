use basecoat_core::attrs::escape_attr;
use basecoat_core::{InputProps, Markup, classes};

/// Renders an `<input>` element with basecoat input classes.
///
/// HTML structure: `<input class="{classes}" type="..." placeholder="..." value="..." {attrs}>`
/// Input is a void element — no closing tag.
pub fn input(props: InputProps) -> Markup {
    let class = classes::input(&props);
    let mut extra = String::new();

    if let Some(type_value) = &props.r#type {
        extra.push_str(&format!(r#" type="{}""#, escape_attr(type_value)));
    }
    if let Some(placeholder) = &props.placeholder {
        extra.push_str(&format!(r#" placeholder="{}""#, escape_attr(placeholder)));
    }
    if let Some(value) = &props.value {
        extra.push_str(&format!(r#" value="{}""#, escape_attr(value)));
    }
    if props.disabled {
        extra.push_str(" disabled");
    }

    let attrs = props.attrs.render();
    Markup::from(format!(r#"<input class="{class}"{extra}{attrs}>"#))
}
