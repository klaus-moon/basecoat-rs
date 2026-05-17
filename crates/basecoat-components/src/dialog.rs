use basecoat_core::{DialogProps, Markup, attrs::escape_attr, classes};

// SVG close icon — same as upstream basecoat (lucide X icon).
const CLOSE_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-x-icon lucide-x"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;

/// Renders a full dialog component (trigger button + `<dialog>` element).
///
/// Upstream HTML structure:
/// ```html
/// <button type="button" onclick="document.getElementById('{id}').showModal()" ...>
///   {trigger}
/// </button>
/// <dialog
///   id="{id}"
///   class="dialog"
///   aria-labelledby="{id}-title"
///   aria-describedby="{id}-description"
///   onclick="if (event.target === this) this.close()"
///   data-dialog
///   data-basecoat-hydrate="dialog"
///   data-basecoat-version="0.1"
/// >
///   <div>
///     <header><h2 id="{id}-title">{title}</h2><p id="{id}-description">{description}</p></header>
///     <section>{children}</section>
///     <footer>{footer}</footer>
///     <button type="button" aria-label="Close dialog" onclick="this.closest('dialog').close()">
///       <!-- X icon -->
///     </button>
///   </div>
/// </dialog>
/// ```
pub fn dialog(props: DialogProps) -> Markup {
    let id = props.id.as_deref().unwrap_or("dialog-1").to_owned();

    let class = classes::dialog(&props);
    let dialog_attrs = props.attrs.render();

    let mut html = String::new();

    // Trigger button
    if let Some(trigger) = &props.trigger {
        html.push_str(&format!(
            r#"<button type="button" data-dialog-trigger="{eid}" onclick="document.getElementById('{eid}').showModal()">{trigger}</button>"#,
            eid = escape_attr(&id),
            trigger = trigger,
        ));
    }

    // onclick overlay close attribute
    let overlay_close = if props.close_on_overlay_click {
        r#" onclick="if (event.target === this) this.close()""#
    } else {
        ""
    };

    // aria-describedby only when description is present
    let aria_describedby = if props.description.is_some() {
        format!(r#" aria-describedby="{}-description""#, escape_attr(&id))
    } else {
        String::new()
    };

    html.push_str(&format!(
        r#"<dialog id="{id}" class="{class}" aria-labelledby="{id}-title"{aria_describedby}{overlay_close} data-dialog data-basecoat-hydrate="dialog" data-basecoat-version="0.1"{dialog_attrs}><div>"#,
        id = escape_attr(&id),
        class = class,
        aria_describedby = aria_describedby,
        overlay_close = overlay_close,
        dialog_attrs = dialog_attrs,
    ));

    // Header (title + description)
    if props.title.is_some() || props.description.is_some() {
        html.push_str("<header>");
        if let Some(title) = &props.title {
            html.push_str(&format!(
                r#"<h2 id="{}-title">{}</h2>"#,
                escape_attr(&id),
                escape_attr(title)
            ));
        }
        if let Some(description) = &props.description {
            html.push_str(&format!(
                r#"<p id="{}-description">{}</p>"#,
                escape_attr(&id),
                escape_attr(description)
            ));
        }
        html.push_str("</header>");
    }

    // Body
    let children = props.children.to_string();
    if !children.is_empty() {
        html.push_str(&format!("<section>{children}</section>"));
    }

    // Footer
    if let Some(footer) = &props.footer {
        html.push_str(&format!("<footer>{footer}</footer>"));
    }

    // Close button
    if props.close_button {
        html.push_str(&format!(
            r#"<button type="button" aria-label="Close dialog" onclick="this.closest('dialog').close()">{CLOSE_ICON}</button>"#
        ));
    }

    html.push_str("</div></dialog>");
    Markup::from(html)
}
