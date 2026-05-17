use basecoat_core::props::tooltip::TooltipSide;
use basecoat_core::{Markup, TooltipProps, attrs::escape_attr, classes};

/// Renders a tooltip trigger element with `data-tooltip` and `data-side` attributes.
///
/// Upstream basecoat implements tooltips CSS-only via `data-tooltip` on the trigger.
/// No wrapper div — the children IS the trigger element (caller provides the button/link).
///
/// HTML structure (wraps children in a `<span>` with tooltip attrs when no children tag
/// is provided; when children IS the trigger, the caller wraps it themselves):
///
/// The canonical approach: emit a `<span>` with the tooltip data attrs wrapping children.
/// ```html
/// <span data-tooltip="{content}" data-side="{side}" class="{class}" {attrs}>
///   {children}
/// </span>
/// ```
pub fn tooltip(props: TooltipProps) -> Markup {
    let class = classes::tooltip(&props);
    let content_escaped = escape_attr(&props.content);
    let side = &props.side;
    let attrs = props.attrs.render();
    let children = &props.children;

    // Only emit data-side when non-default (top is the CSS default so we always emit it
    // for explicitness, matching upstream behaviour of always setting the attribute when
    // the side is known).
    let side_attr = match side {
        TooltipSide::Top => String::new(), // top is CSS default; omit for minimal output
        other => format!(r#" data-side="{}""#, other),
    };

    let class_attr = if class.is_empty() {
        String::new()
    } else {
        format!(r#" class="{class}""#)
    };

    Markup::from(format!(
        r#"<span data-tooltip="{content_escaped}"{side_attr}{class_attr}{attrs}>{children}</span>"#
    ))
}
