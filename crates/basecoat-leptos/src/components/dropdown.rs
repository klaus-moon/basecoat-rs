use basecoat_core::classes::dropdown::dropdown as dropdown_class;
use basecoat_core::props::dropdown::DropdownProps as CoreProps;
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Dropdown component.
///
/// Emits a native `<details class="dropdown-menu">` element with the
/// `data-basecoat-hydrate="dropdown"` + `data-basecoat-version="0.2"` markers
/// so the WASM controller (`basecoat-controllers`) can attach floating
/// positioning, roving-tabindex keyboard navigation, and dismiss behavior.
///
/// The `id` is **required** for the controller to find the element.
///
/// ## Compound sub-components
///
/// - [`DropdownTrigger`] — the `<summary>` button that opens the menu
/// - [`DropdownMenu`] — the `<div role="menu">` floating container
/// - [`DropdownItem`] — an individual `<button role="menuitem">`
#[component]
pub fn Dropdown(
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Floating placement string passed to `@floating-ui/dom`
    /// (default `"bottom-start"`).
    #[prop(optional, into)]
    placement: Option<Cow<'static, str>>,
    /// Extra CSS classes on the outer `<details>`.
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
        dropdown_class(&props)
    });
    let placement_value = placement
        .map(|p| p.to_string())
        .unwrap_or_else(|| "bottom-start".to_string());

    view! {
        <details
            class=class_str
            id=id.map(|s| s.to_string())
            data-dropdown=""
            data-basecoat-hydrate="dropdown"
            data-basecoat-version="0.2"
            data-placement=placement_value
        >
            {children()}
        </details>
    }
}

/// The `<summary>` trigger for a [`Dropdown`].
///
/// Renders with `aria-haspopup="menu"` and `aria-expanded="false"`; the
/// controller mirrors the expanded state to match the `<details>` `open`
/// attribute on toggle.
#[component]
pub fn DropdownTrigger(
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    view! {
        <summary
            class=class.map(|c| c.to_string())
            aria-haspopup="menu"
            aria-expanded="false"
        >
            {children()}
        </summary>
    }
}

/// The floating menu container (`<div role="menu">`) inside a [`Dropdown`].
#[component]
pub fn DropdownMenu(
    /// Optional aria-label for screen readers.
    #[prop(optional, into)]
    label: Option<Cow<'static, str>>,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class.map(|c| c.to_string())
            role="menu"
            aria-label=label.map(|l| l.to_string())
        >
            {children()}
        </div>
    }
}

/// An individual menu item inside a [`DropdownMenu`].
#[component]
pub fn DropdownItem(
    /// Optional value attribute rendered as `data-value="..."`.
    #[prop(optional, into)]
    value: Option<Cow<'static, str>>,
    /// When true, the item is disabled.
    #[prop(default = false)]
    disabled: bool,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=class.map(|c| c.to_string())
            role="menuitem"
            tabindex="-1"
            type="button"
            data-value=value.map(|v| v.to_string())
            disabled=disabled
            aria-disabled=if disabled { Some("true") } else { None }
        >
            {children()}
        </button>
    }
}
