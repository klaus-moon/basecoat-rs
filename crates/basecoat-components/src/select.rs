use basecoat_core::attrs::escape_attr;
use basecoat_core::classes::select::select as select_class;
use basecoat_core::props::select::{SelectOption, SelectProps};
use basecoat_core::Markup;

/// Renders a select component: hidden native `<select>` (form source of
/// truth) + visible trigger button + floating listbox.
///
/// Rendered HTML structure:
/// ```html
/// <div class="select" id="{id}-root" data-basecoat-hydrate="select" data-basecoat-version="0.2">
///   <select class="select-native" hidden name="{name}">
///     <option value="a">Apple</option>
///     ...
///   </select>
///   <button
///     type="button"
///     class="select-trigger"
///     id="{id}"
///     data-select-trigger
///     aria-haspopup="listbox"
///     aria-expanded="false"
///   ><span data-select-value>{selected_label}</span></button>
///   <div role="listbox" data-select-listbox tabindex="-1" hidden>
///     <button type="button" role="option" data-value="a" tabindex="-1">Apple</button>
///     ...
///   </div>
/// </div>
/// ```
pub fn select(props: SelectProps) -> Markup {
    let id = props.id.as_deref().unwrap_or("select-1").to_owned();
    let class = select_class(&props);
    let wrapper_attrs = props.attrs.render();

    let selected_value = props.value.as_deref().unwrap_or_default();
    let placeholder = props.placeholder.as_deref().unwrap_or("Select an option");

    let selected_label = props
        .options
        .iter()
        .find(|o| o.value == selected_value)
        .map(|o| o.label.as_ref())
        .unwrap_or(placeholder);

    let name_attr = match &props.name {
        Some(n) => format!(r#" name="{}""#, escape_attr(n)),
        None => String::new(),
    };
    let disabled_attr_native = if props.disabled { " disabled" } else { "" };
    let disabled_attr_trigger = if props.disabled { " disabled" } else { "" };
    let aria_label_attr = match &props.label {
        Some(l) => format!(r#" aria-label="{}""#, escape_attr(l)),
        None => String::new(),
    };

    let mut html = String::new();

    // Outer wrapper (carries the .select class and hydration markers).
    html.push_str(&format!(
        r#"<div class="{class}" id="{id_root}" data-basecoat-hydrate="select" data-basecoat-version="0.2"{wrapper_attrs}>"#,
        class = class,
        id_root = escape_attr(&format!("{id}-root")),
        wrapper_attrs = wrapper_attrs,
    ));

    // Hidden native <select> — source of truth for form submission.
    html.push_str(&format!(
        r#"<select class="select-native" hidden{name}{disabled} id="{id_native}" data-select-native>"#,
        name = name_attr,
        disabled = disabled_attr_native,
        id_native = escape_attr(&format!("{id}-native")),
    ));
    for option in &props.options {
        push_native_option(&mut html, option, selected_value);
    }
    html.push_str("</select>");

    // Visible trigger button.
    html.push_str(&format!(
        r#"<button type="button" class="select-trigger" id="{eid}" data-select-trigger aria-haspopup="listbox" aria-expanded="false" aria-controls="{listbox_id}"{aria_label}{disabled}>"#,
        eid = escape_attr(&id),
        listbox_id = escape_attr(&format!("{id}-listbox")),
        aria_label = aria_label_attr,
        disabled = disabled_attr_trigger,
    ));
    html.push_str(&format!(
        r#"<span data-select-value>{}</span>"#,
        escape_attr(selected_label)
    ));
    html.push_str("</button>");

    // Floating listbox.
    html.push_str(&format!(
        r#"<div id="{listbox_id}" role="listbox" data-select-listbox tabindex="-1" hidden aria-labelledby="{eid}">"#,
        listbox_id = escape_attr(&format!("{id}-listbox")),
        eid = escape_attr(&id),
    ));

    // Optional inline children rendered ahead of the option buttons.
    let children = props.children.to_string();
    if !children.is_empty() {
        html.push_str(&children);
    }

    for option in &props.options {
        push_listbox_option(&mut html, option, selected_value);
    }

    html.push_str("</div></div>");
    Markup::from(html)
}

fn push_native_option(html: &mut String, option: &SelectOption, selected_value: &str) {
    let selected = if option.value == selected_value {
        " selected"
    } else {
        ""
    };
    let disabled = if option.disabled { " disabled" } else { "" };
    html.push_str(&format!(
        r#"<option value="{value}"{selected}{disabled}>{label}</option>"#,
        value = escape_attr(&option.value),
        selected = selected,
        disabled = disabled,
        label = escape_attr(&option.label),
    ));
}

fn push_listbox_option(html: &mut String, option: &SelectOption, selected_value: &str) {
    let is_selected = option.value == selected_value;
    let aria_selected = if is_selected { "true" } else { "false" };
    let disabled_attrs = if option.disabled {
        r#" aria-disabled="true" disabled"#
    } else {
        ""
    };
    let tabindex = if is_selected { "0" } else { "-1" };
    html.push_str(&format!(
        r#"<button type="button" role="option" data-value="{value}" aria-selected="{aria_selected}" tabindex="{tabindex}"{disabled}>{label}</button>"#,
        value = escape_attr(&option.value),
        aria_selected = aria_selected,
        tabindex = tabindex,
        disabled = disabled_attrs,
        label = escape_attr(&option.label),
    ));
}
