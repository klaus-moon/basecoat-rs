use basecoat_core::props::tooltip::TooltipSide;
use basecoat_core::{TooltipProps as CoreProps, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Tooltip component.
///
/// Basecoat tooltips are CSS-only via `data-tooltip` and `data-side` attributes —
/// no JS controller needed. The component wraps its `children` (the trigger element)
/// with these attributes.
#[component]
pub fn Tooltip(
    /// The tooltip text content.
    #[prop(into)]
    content: Cow<'static, str>,
    /// Tooltip position relative to the trigger.
    #[prop(optional, into)]
    side: Signal<TooltipSide>,
    /// Extra CSS classes on the wrapper span.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
    children: Children,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            class: extra,
            content: Cow::Borrowed(""),
            ..Default::default()
        };
        classes::tooltip(&props)
    });
    let side_str = Signal::derive(move || side.get().to_string());

    view! {
        <span
            class=class_str
            data-tooltip=content.to_string()
            data-side=side_str
        >
            {children()}
        </span>
    }
}
