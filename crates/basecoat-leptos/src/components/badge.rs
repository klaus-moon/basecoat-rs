use basecoat_core::{BadgeProps as CoreProps, BadgeVariant, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Badge component.
///
/// Renders a `<span>` with the canonical `.badge[-variant]` class string.
#[component]
pub fn Badge(
    /// Visual variant.
    #[prop(optional, into)]
    variant: Signal<BadgeVariant>,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
    children: Children,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            variant: variant.get(),
            class: extra,
            ..Default::default()
        };
        classes::badge(&props)
    });
    view! { <span class=class_str>{children()}</span> }
}
