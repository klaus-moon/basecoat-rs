use basecoat_core::attrs::escape_attr;
use basecoat_core::{LabelProps, Markup, classes};

/// Renders a `<label>` element with basecoat label classes.
///
/// HTML structure: `<label class="{classes}" for="...">{children}</label>`
pub fn label(props: LabelProps) -> Markup {
    let class = classes::label(&props);
    let mut extra = String::new();

    if let Some(for_value) = &props.r#for {
        extra.push_str(&format!(r#" for="{}""#, escape_attr(for_value)));
    }

    let attrs = props.attrs.render();
    let children = &props.children;
    Markup::from(format!(
        r#"<label class="{class}"{extra}{attrs}>{children}</label>"#
    ))
}
