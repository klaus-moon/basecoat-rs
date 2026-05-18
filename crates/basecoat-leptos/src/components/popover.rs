use basecoat_core::classes::popover::popover as popover_class;
use basecoat_core::props::popover::{PopoverPlacement, PopoverProps as CoreProps};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Popover component.
///
/// Emits a `<details class="popover">` root with `data-basecoat-hydrate="popover"`
/// and `data-basecoat-version="0.2"` so the WASM controller (`basecoat-controllers`)
/// can attach floating-ui positioning and dismissal behavior.
///
/// The `id` is **required** for the controller to find the element.
///
/// ## Compound components
///
/// Use the standalone [`PopoverTrigger`] and [`PopoverContent`] sub-components
/// for full control over trigger and content markup.
#[component]
pub fn Popover(
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Floating-ui placement (default `bottom`).
    #[prop(optional, into)]
    placement: Signal<PopoverPlacement>,
    /// Distance in pixels between the trigger and the content panel (default 8.0).
    #[prop(default = 8.0)]
    offset_px: f64,
    /// Whether to render a `<div data-popover-arrow>` inside the content panel.
    #[prop(default = false)]
    arrow: bool,
    /// Extra CSS classes appended to the `popover` base class.
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
        popover_class(&props)
    });
    let placement_str = Signal::derive(move || placement.get().as_str().to_string());

    view! {
        <details
            class=class_str
            id=id.map(|s| s.to_string())
            data-basecoat-hydrate="popover"
            data-basecoat-version="0.2"
            data-popover=""
            data-placement=placement_str
            data-offset=offset_px.to_string()
        >
            {children()}
            {arrow.then(|| view! { <div data-popover-arrow=""></div> })}
        </details>
    }
}

/// The trigger element for a [`Popover`] — emits a `<summary>` with the
/// `aria-haspopup="dialog"` semantics the controller expects.
#[component]
pub fn PopoverTrigger(
    /// Optional `aria-controls` target — usually `"{popover-id}-content"`.
    #[prop(optional, into)]
    controls: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    view! {
        <summary
            aria-haspopup="dialog"
            aria-controls=controls.map(|s| s.to_string())
            aria-expanded="false"
        >
            {children()}
        </summary>
    }
}

/// The content panel for a [`Popover`] — emits a `<div role="dialog">`.
///
/// The `id` should match the `aria-controls` value used on the corresponding
/// [`PopoverTrigger`], conventionally `"{popover-id}-content"`.
#[component]
pub fn PopoverContent(
    /// Unique DOM id — should match `aria-controls` on the trigger.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Extra CSS classes on the content panel.
    #[prop(optional, into)]
    class: Option<Signal<Cow<'static, str>>>,
    children: Children,
) -> impl IntoView {
    let class_str = Signal::derive(move || class.as_ref().map(|s| s.get().to_string()));

    view! {
        <div
            id=id.map(|s| s.to_string())
            role="dialog"
            tabindex="-1"
            class=class_str
        >
            {children()}
        </div>
    }
}
