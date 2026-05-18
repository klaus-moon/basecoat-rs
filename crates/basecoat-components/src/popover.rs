use basecoat_core::classes::popover::popover as popover_class;
use basecoat_core::props::popover::PopoverProps;
use basecoat_core::{Markup, attrs::escape_attr};

/// Renders a popover (`<details class="popover">` with `<summary>` trigger and
/// `<div role="dialog">` content).
///
/// Upstream HTML structure:
/// ```html
/// <details
///   id="{id}"
///   class="popover"
///   data-basecoat-hydrate="popover"
///   data-basecoat-version="0.2"
///   data-popover
///   data-placement="{placement}"
///   data-offset="{offset_px}"
/// >
///   <summary aria-haspopup="dialog" aria-controls="{id}-content" aria-expanded="false">
///     {trigger}
///   </summary>
///   <div id="{id}-content" role="dialog" tabindex="-1">
///     {children}
///     <div data-popover-arrow></div>   <!-- only when props.arrow is true -->
///   </div>
/// </details>
/// ```
pub fn popover(props: PopoverProps) -> Markup {
    let id = props.id.as_deref().unwrap_or("popover-1").to_owned();
    let id_attr = escape_attr(&id);
    let content_id = format!("{id}-content");
    let content_id_attr = escape_attr(&content_id);
    let class = popover_class(&props);
    let attrs = props.attrs.render();
    let placement = props.placement.as_str();
    let offset = props.offset_px;

    let trigger_html = props
        .trigger
        .as_ref()
        .map(|m| m.to_string())
        .unwrap_or_default();
    let children = props.children.to_string();

    let arrow_html = if props.arrow {
        r#"<div data-popover-arrow></div>"#
    } else {
        ""
    };

    Markup::from(format!(
        r#"<details id="{id_attr}" class="{class}" data-basecoat-hydrate="popover" data-basecoat-version="0.2" data-popover data-placement="{placement}" data-offset="{offset}"{attrs}><summary aria-haspopup="dialog" aria-controls="{content_id_attr}" aria-expanded="false">{trigger_html}</summary><div id="{content_id_attr}" role="dialog" tabindex="-1">{children}{arrow_html}</div></details>"#
    ))
}
