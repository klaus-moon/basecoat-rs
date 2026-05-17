use basecoat_core::{TabsOrientation, TabsProps as CoreProps, classes};
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Tabs component.
///
/// Emits a `<div class="tabs">` with `data-basecoat-hydrate="tabs"` and
/// `data-basecoat-version="0.1"` so the WASM controller can attach.
///
/// The `id` is required for the controller.
///
/// ## Compound sub-components
///
/// For custom tab composition use:
/// - [`TabsList`] — the `<nav>` tab button bar
/// - [`TabsTab`] — an individual tab button
/// - [`TabsPanel`] — an individual tab panel
#[component]
pub fn Tabs(
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Orientation: horizontal (default) or vertical.
    #[prop(optional, into)]
    orientation: Signal<TabsOrientation>,
    /// Extra CSS classes on the outer container.
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
        classes::tabs(&props)
    });
    let orientation_str = Signal::derive(move || orientation.get().to_string());

    view! {
        <div
            class=class_str
            id=id.map(|s| s.to_string())
            data-basecoat-hydrate="tabs"
            data-basecoat-version="0.1"
            aria-orientation=orientation_str
        >
            {children()}
        </div>
    }
}

/// The tab button list (`<nav>` element) inside a Tabs component.
#[component]
pub fn TabsList(
    /// aria-orientation for the nav element.
    #[prop(optional, into)]
    orientation: Signal<TabsOrientation>,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    let orientation_str = Signal::derive(move || orientation.get().to_string());
    view! {
        <nav
            class=class.map(|c| format!("tabs-list {c}")).unwrap_or_else(|| "tabs-list".to_string())
            role="tablist"
            aria-orientation=orientation_str
        >
            {children()}
        </nav>
    }
}

/// An individual tab trigger button.
#[component]
pub fn TabsTab(
    /// `aria-controls` — must match the id of the corresponding [`TabsPanel`].
    #[prop(into)]
    controls: Cow<'static, str>,
    /// Whether this tab is selected by default.
    #[prop(default = false)]
    selected: bool,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=class.map(|c| format!("tabs-tab {c}")).unwrap_or_else(|| "tabs-tab".to_string())
            role="tab"
            aria-controls=controls.to_string()
            aria-selected=if selected { "true" } else { "false" }
            type="button"
        >
            {children()}
        </button>
    }
}

/// An individual tab panel (content area).
#[component]
pub fn TabsPanel(
    /// Must match the `controls` attribute on the corresponding [`TabsTab`].
    #[prop(into)]
    id: Cow<'static, str>,
    /// Whether this panel is visible by default.
    #[prop(default = false)]
    selected: bool,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class.map(|c| format!("tabs-panel {c}")).unwrap_or_else(|| "tabs-panel".to_string())
            id=id.to_string()
            role="tabpanel"
            hidden=if selected { None::<&str> } else { Some("") }
        >
            {children()}
        </div>
    }
}
