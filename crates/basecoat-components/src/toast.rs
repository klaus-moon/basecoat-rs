use basecoat_core::{Markup, ToastCategory, ToastProps, ToasterProps, attrs::escape_attr, classes};

// Inline SVG icons mirroring upstream basecoat toast icons.
const ICON_SUCCESS: &str = r#"<svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#;
const ICON_ERROR: &str = r#"<svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>"#;
const ICON_INFO: &str = r#"<svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;
const ICON_WARNING: &str = r#"<svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#;

/// Renders the toaster container `<div data-toaster>` that holds toast messages.
///
/// HTML structure:
/// ```html
/// <div id="{id}" class="toaster" data-toaster {attrs}></div>
/// ```
pub fn toaster(props: ToasterProps) -> Markup {
    let id = &props.id;
    let class = match &props.class {
        Some(c) if !c.is_empty() => format!("toaster {c}"),
        _ => "toaster".to_string(),
    };
    let attrs = props.attrs.render();
    Markup::from(format!(
        r#"<div id="{id}" class="{class}" data-toaster data-basecoat-hydrate="toast" data-basecoat-version="0.1"{attrs}></div>"#,
        id = escape_attr(id),
        class = class,
        attrs = attrs,
    ))
}

/// Renders a single toast notification.
///
/// Upstream HTML structure:
/// ```html
/// <div
///   class="toast"
///   role="alert|status"
///   aria-atomic="true"
///   aria-hidden="false"
///   data-category="{category}"
///   data-toast
///   data-basecoat-hydrate="toast"
///   data-basecoat-version="0.1"
/// >
///   <div class="toast-content">
///     <!-- SVG icon -->
///     <section>
///       <h2>{title}</h2>
///       <p>{description}</p>
///     </section>
///   </div>
/// </div>
/// ```
pub fn toast(props: ToastProps) -> Markup {
    let class = classes::toast(&props);
    let category_str = props.category.to_string();
    let role = if props.category == ToastCategory::Error {
        "alert"
    } else {
        "status"
    };
    let attrs = props.attrs.render();

    let icon = match props.category {
        ToastCategory::Success => ICON_SUCCESS,
        ToastCategory::Error => ICON_ERROR,
        ToastCategory::Info => ICON_INFO,
        ToastCategory::Warning => ICON_WARNING,
    };

    let mut content = String::new();
    content.push_str(icon);
    content.push_str("<section>");
    if let Some(title) = &props.title {
        content.push_str(&format!("<h2>{}</h2>", escape_attr(title)));
    }
    if let Some(description) = &props.description {
        content.push_str(&format!("<p>{}</p>", escape_attr(description)));
    }
    content.push_str("</section>");

    Markup::from(format!(
        r#"<div class="{class}" role="{role}" aria-atomic="true" aria-hidden="false" data-category="{category}" data-toast data-basecoat-hydrate="toast" data-basecoat-version="0.1"{attrs}><div class="toast-content">{content}</div></div>"#,
        class = class,
        role = role,
        category = category_str,
        attrs = attrs,
        content = content,
    ))
}
