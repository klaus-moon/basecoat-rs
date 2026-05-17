use basecoat_core::{ButtonProps as CoreProps, ButtonSize, ButtonVariant, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Button component.
///
/// Shares class-string computation with `basecoat_core::classes::button`.
/// Pass-through attributes: use Leptos's `attr:` syntax on the wrapping
/// `<Button>` element — see crate README for a worked example (Option B).
#[component]
pub fn Button(
    /// Visual variant. Accepts static values or reactive signals.
    #[prop(optional, into)]
    variant: Signal<ButtonVariant>,
    /// Size variant. Accepts static values or reactive signals.
    #[prop(optional, into)]
    size: Signal<ButtonSize>,
    /// Extra CSS classes appended after the canonical class string.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
    children: Children,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            variant: variant.get(),
            size: size.get(),
            class: extra,
            attrs: Default::default(),
            children: Default::default(),
        };
        classes::button(&props)
    });
    view! { <button class=class_str>{children()}</button> }
}
