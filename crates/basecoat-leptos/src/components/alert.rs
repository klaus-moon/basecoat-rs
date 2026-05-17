use basecoat_core::{AlertProps as CoreProps, AlertVariant, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Alert component.
///
/// Renders a `<div role="alert">` with the canonical `.alert[-destructive]` class string.
#[component]
pub fn Alert(
    /// Visual variant.
    #[prop(optional, into)]
    variant: Signal<AlertVariant>,
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
        classes::alert(&props)
    });
    view! {
        <div class=class_str role="alert">
            {children()}
        </div>
    }
}
