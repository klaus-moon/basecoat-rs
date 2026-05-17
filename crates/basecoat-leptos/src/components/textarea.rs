use basecoat_core::{TextareaProps as CoreProps, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Textarea component.
///
/// Renders a `<textarea>` with the canonical `.textarea` class string.
#[component]
pub fn Textarea(
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<Signal<Cow<'static, str>>>,
    /// Disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            class: extra,
            ..Default::default()
        };
        classes::textarea(&props)
    });
    let placeholder_val = placeholder.map(|s| Signal::derive(move || s.get().to_string()));

    view! {
        <textarea
            class=class_str
            placeholder=placeholder_val
            disabled=disabled
        />
    }
}
