use basecoat_core::{CardProps as CoreProps, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Card component.
///
/// Renders a `<div>` with the canonical `.card` class string.
#[component]
pub fn Card(
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
    children: Children,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            class: extra,
            ..Default::default()
        };
        classes::card(&props)
    });
    view! { <div class=class_str>{children()}</div> }
}
