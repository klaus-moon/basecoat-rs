use basecoat_core::{LabelProps as CoreProps, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Label component.
///
/// Renders a `<label>` with the canonical `.label` class string.
#[component]
pub fn Label(
    /// `for` attribute linking to an input id.
    #[prop(optional, into)]
    r#for: Option<Signal<Cow<'static, str>>>,
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
        classes::label(&props)
    });
    let for_val = r#for.map(|s| Signal::derive(move || s.get().to_string()));

    view! {
        <label class=class_str for=for_val>
            {children()}
        </label>
    }
}
