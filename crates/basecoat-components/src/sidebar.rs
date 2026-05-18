use basecoat_core::classes::sidebar::sidebar as sidebar_class;
use basecoat_core::props::sidebar::SidebarProps;
use basecoat_core::{Markup, attrs::escape_attr};

/// Renders a responsive sidebar `<aside>` element.
///
/// Upstream HTML structure:
/// ```html
/// <aside
///   id="{id}"
///   class="sidebar"
///   data-state="expanded"
///   data-basecoat-hydrate="sidebar"
///   data-basecoat-version="0.2"
/// >
///   <header class="sidebar-header">{header}</header>
///   <nav class="sidebar-nav">{children}</nav>
///   <footer class="sidebar-footer">{footer}</footer>
/// </aside>
/// ```
///
/// The sibling toggle button (rendered separately by the consumer, e.g. via
/// [`sidebar_toggle`]) flips the `data-state` between `expanded`/`collapsed`
/// and updates its own `aria-expanded`.
pub fn sidebar(props: SidebarProps) -> Markup {
    let id = props.id.as_deref().unwrap_or("sidebar-1").to_owned();
    let class = sidebar_class(&props);
    let attrs = props.attrs.render();
    let state = if props.default_open {
        "expanded"
    } else {
        "collapsed"
    };

    let mut html = String::new();
    html.push_str(&format!(
        r#"<aside id="{id}" class="{class}" data-state="{state}" data-basecoat-hydrate="sidebar" data-basecoat-version="0.2"{attrs}>"#,
        id = escape_attr(&id),
        class = class,
        state = state,
        attrs = attrs,
    ));

    if let Some(header) = &props.header {
        html.push_str(&format!(r#"<header class="sidebar-header">{header}</header>"#));
    }

    let children = props.children.to_string();
    html.push_str(&format!(r#"<nav class="sidebar-nav">{children}</nav>"#));

    if let Some(footer) = &props.footer {
        html.push_str(&format!(r#"<footer class="sidebar-footer">{footer}</footer>"#));
    }

    html.push_str("</aside>");
    Markup::from(html)
}

/// Renders the sibling toggle button for a [`sidebar`] with the given id.
///
/// The button carries `data-sidebar-toggle="{target_id}"`,
/// `aria-controls="{target_id}"`, and `aria-expanded` reflecting the initial
/// open state. The WASM controller wires the click handler.
pub fn sidebar_toggle(target_id: &str, label: &str, default_open: bool) -> Markup {
    let expanded = if default_open { "true" } else { "false" };
    Markup::from(format!(
        r#"<button type="button" data-sidebar-toggle="{tid}" aria-controls="{tid}" aria-expanded="{expanded}">{label}</button>"#,
        tid = escape_attr(target_id),
        expanded = expanded,
        label = escape_attr(label),
    ))
}
