use basecoat_core::{ToastCategory, ToastProps as CoreProps, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Toaster container.
///
/// The toaster container holds individual [`Toast`] items and carries the
/// `data-basecoat-hydrate="toast"` marker so the WASM controller can attach.
#[component]
pub fn Toaster(
    /// DOM id for the toaster container (default: "toaster").
    #[prop(optional, into, default = Cow::Borrowed("toaster"))]
    id: Cow<'static, str>,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    let base = "toaster";
    let class_str = match class {
        Some(ref extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base.to_string(),
    };
    view! {
        <div
            class=class_str
            id=id.to_string()
            data-toaster=""
            data-basecoat-hydrate="toast"
            data-basecoat-version="0.1"
            aria-live="polite"
            aria-atomic="false"
        >
            {children()}
        </div>
    }
}

/// Leptos wrapper for a single basecoat Toast message.
///
/// Place inside a [`Toaster`].
#[component]
pub fn Toast(
    /// Category (success/error/info/warning).
    #[prop(optional, into)]
    category: Signal<ToastCategory>,
    /// Toast title text.
    #[prop(optional, into)]
    title: Option<Cow<'static, str>>,
    /// Toast description text.
    #[prop(optional, into)]
    description: Option<Cow<'static, str>>,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
    /// Optional inner content rendered after title and description.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let class_str = Signal::derive(move || {
        let extra = class.as_ref().map(|s| s.get());
        let props = CoreProps {
            category: category.get(),
            class: extra,
            ..Default::default()
        };
        classes::toast(&props)
    });
    let category_str = Signal::derive(move || category.get().to_string());

    view! {
        <div
            class=class_str
            data-category=category_str
            role="status"
            aria-live="assertive"
            aria-atomic="true"
        >
            {title.map(|t| view! { <p class="toast-title">{t}</p> })}
            {description.map(|d| view! { <p class="toast-description">{d}</p> })}
            {children.map(|c| c())}
        </div>
    }
}
