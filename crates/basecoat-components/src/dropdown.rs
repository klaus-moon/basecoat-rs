use basecoat_core::Markup;
use basecoat_core::attrs::escape_attr;
use basecoat_core::classes::dropdown::dropdown as dropdown_class;
use basecoat_core::props::dropdown::{DropdownItem, DropdownProps};

/// Renders a dropdown component as a `<details>` element with a `<summary>`
/// trigger and a floating `<div role="menu">` populated with menuitem buttons.
///
/// Rendered HTML structure:
///
/// ```html
/// <details
///   id="{id}"
///   class="dropdown-menu"
///   data-dropdown
///   data-basecoat-hydrate="dropdown"
///   data-basecoat-version="0.2"
///   data-placement="bottom-start"
/// >
///   <summary aria-haspopup="menu" aria-expanded="false">{trigger}</summary>
///   <div role="menu" aria-label="{menu_label?}">
///     <button type="button" role="menuitem" tabindex="-1"
///             data-value="{item.value?}"
///             aria-disabled="{item.disabled}"
///             [disabled]>{item.label}</button>
///     ...
///   </div>
/// </details>
/// ```
pub fn dropdown(props: DropdownProps) -> Markup {
    let id = props.id.as_deref().unwrap_or("dropdown-1").to_owned();
    let class = dropdown_class(&props);
    let attrs = props.attrs.render();
    let placement = props
        .placement
        .as_deref()
        .map(str::to_owned)
        .unwrap_or_else(|| "bottom-start".to_string());

    let mut html = String::new();

    html.push_str(&format!(
        concat!(
            r#"<details id="{id}" class="{class}" data-dropdown "#,
            r#"data-basecoat-hydrate="dropdown" data-basecoat-version="0.2" "#,
            r#"data-placement="{placement}"{attrs}>"#,
        ),
        id = escape_attr(&id),
        class = escape_attr(&class),
        placement = escape_attr(&placement),
        attrs = attrs,
    ));

    // <summary> trigger
    let trigger_inner = props
        .trigger
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_default();
    html.push_str(&format!(
        r#"<summary aria-haspopup="menu" aria-expanded="false">{trigger}</summary>"#,
        trigger = trigger_inner,
    ));

    // Menu container
    let menu_label_attr = match props.menu_label.as_deref() {
        Some(label) => format!(r#" aria-label="{}""#, escape_attr(label)),
        None => String::new(),
    };
    html.push_str(&format!(
        r#"<div role="menu"{menu_label}>"#,
        menu_label = menu_label_attr,
    ));

    for item in &props.items {
        html.push_str(&render_item(item));
    }

    // Any extra children are appended after the generated items so callers
    // can interleave separators, headings, custom markup.
    let children = props.children.to_string();
    if !children.is_empty() {
        html.push_str(&children);
    }

    html.push_str("</div></details>");
    Markup::from(html)
}

fn render_item(item: &DropdownItem) -> String {
    let data_value = match item.value.as_deref() {
        Some(value) => format!(r#" data-value="{}""#, escape_attr(value)),
        None => String::new(),
    };
    let disabled_attr = if item.disabled {
        r#" disabled aria-disabled="true""#
    } else {
        ""
    };
    format!(
        concat!(
            r#"<button type="button" role="menuitem" tabindex="-1""#,
            r#"{data_value}{disabled}>{label}</button>"#,
        ),
        data_value = data_value,
        disabled = disabled_attr,
        label = escape_attr(&item.label),
    )
}
