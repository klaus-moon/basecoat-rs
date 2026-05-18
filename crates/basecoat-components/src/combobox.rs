use basecoat_core::attrs::escape_attr;
use basecoat_core::classes::combobox::combobox as combobox_class;
use basecoat_core::Markup;
use basecoat_core::props::combobox::ComboboxProps;

/// Renders the canonical basecoat combobox markup:
///
/// ```html
/// <div class="select" id="{id}" data-combobox
///      data-basecoat-hydrate="combobox"
///      data-basecoat-version="0.2">
///   <input type="text" role="combobox"
///          aria-controls="{id}-listbox"
///          aria-expanded="false"
///          aria-autocomplete="list"
///          autocomplete="off"
///          data-combobox-input
///          [name="..."] [placeholder="..."] [value="..."] [disabled]>
///   <div role="listbox" id="{id}-listbox" data-combobox-listbox hidden>
///     <button type="button" role="option"
///             id="{id}-option-{idx}"
///             data-value="{value}"
///             tabindex="-1">{label}</button>
///     ...
///   </div>
/// </div>
/// ```
///
/// The matching WASM controller (`basecoat-controllers/combobox`) wires
/// filtering, `aria-activedescendant`, dismiss-on-outside-click, and
/// floating-UI positioning.
pub fn combobox(props: ComboboxProps) -> Markup {
    let id = props.id.as_deref().unwrap_or("combobox-1").to_owned();
    let class = combobox_class(&props);
    let attrs = props.attrs.render();
    let listbox_id = format!("{id}-listbox");
    let eid = escape_attr(&id);
    let elist = escape_attr(&listbox_id);

    let mut html = String::new();

    html.push_str(&format!(
        r#"<div class="{class}" id="{eid}" data-combobox data-basecoat-hydrate="combobox" data-basecoat-version="0.2"{attrs}>"#
    ));

    // Input.
    html.push_str(&format!(
        r#"<input type="text" role="combobox" aria-controls="{elist}" aria-expanded="false" aria-autocomplete="list" autocomplete="off" data-combobox-input"#
    ));
    if let Some(name) = &props.name {
        html.push_str(&format!(r#" name="{}""#, escape_attr(name)));
    }
    if let Some(placeholder) = &props.placeholder {
        html.push_str(&format!(r#" placeholder="{}""#, escape_attr(placeholder)));
    }
    if let Some(value) = &props.value {
        html.push_str(&format!(r#" value="{}""#, escape_attr(value)));
    }
    if props.disabled {
        html.push_str(" disabled");
    }
    html.push('>');

    // Listbox.
    html.push_str(&format!(
        r#"<div role="listbox" id="{elist}" data-combobox-listbox hidden>"#
    ));
    for (idx, option) in props.options.iter().enumerate() {
        html.push_str(&format!(
            r#"<button type="button" role="option" id="{eid}-option-{idx}" data-value="{value}" tabindex="-1">{label}</button>"#,
            value = escape_attr(&option.value),
            label = escape_attr(&option.label),
        ));
    }
    html.push_str("</div>");

    html.push_str("</div>");
    Markup::from(html)
}
