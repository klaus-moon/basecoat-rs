use basecoat_core::{InputProps as CoreProps, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Input component.
///
/// Renders an `<input>` with the canonical `.input` class string.
/// Extra HTML attributes (name, id, aria-*) use Leptos `attr:` syntax.
#[component]
pub fn Input(
    /// HTML `type` attribute (text, email, password, etc.)
    #[prop(optional, into)]
    r#type: Option<Signal<Cow<'static, str>>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<Signal<Cow<'static, str>>>,
    /// Controlled value.
    #[prop(optional, into)]
    value: Option<Signal<Cow<'static, str>>>,
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
        classes::input(&props)
    });
    let type_val = r#type.map(|s| Signal::derive(move || s.get().to_string()));
    let placeholder_val = placeholder.map(|s| Signal::derive(move || s.get().to_string()));
    let value_val = value.map(|s| Signal::derive(move || s.get().to_string()));

    view! {
        <input
            class=class_str
            type=type_val
            placeholder=placeholder_val
            value=value_val
            disabled=disabled
        />
    }
}
