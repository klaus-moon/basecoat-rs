use basecoat_core::classes::sidebar::sidebar as sidebar_class;
use basecoat_core::props::sidebar::SidebarProps as CoreProps;
use leptos::prelude::*;
use std::borrow::Cow;

/// Leptos wrapper for the basecoat Sidebar component.
///
/// Emits an `<aside>` element with the canonical `.sidebar` class and the
/// `data-basecoat-hydrate="sidebar"` + `data-basecoat-version="0.2"` markers
/// so the WASM controller (`basecoat-controllers`) can attach.
///
/// The `id` is **required** because it is the localStorage persistence key
/// and the target of the sibling [`SidebarToggle`] button's `aria-controls`.
///
/// ## Compound sub-components
///
/// - [`SidebarHeader`] — `<header class="sidebar-header">`
/// - [`SidebarNav`] — `<nav class="sidebar-nav">`
/// - [`SidebarFooter`] — `<footer class="sidebar-footer">`
/// - [`SidebarToggle`] — the sibling toggle button (NOT a child of the aside)
#[component]
pub fn Sidebar(
    /// Unique DOM id — required for the WASM controller and localStorage key.
    #[prop(optional, into)]
    id: Option<Cow<'static, str>>,
    /// Initial expanded state. Defaults to `true`. The controller reconciles
    /// this against `localStorage` on hydrate.
    #[prop(default = true)]
    default_open: bool,
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
        sidebar_class(&props)
    });
    let state = if default_open { "expanded" } else { "collapsed" };

    view! {
        <aside
            class=class_str
            id=id.map(|s| s.to_string())
            data-state=state
            data-basecoat-hydrate="sidebar"
            data-basecoat-version="0.2"
        >
            {children()}
        </aside>
    }
}

/// `<header class="sidebar-header">` slot.
#[component]
pub fn SidebarHeader(
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    let class_str = class
        .map(|c| format!("sidebar-header {c}"))
        .unwrap_or_else(|| "sidebar-header".to_string());
    view! { <header class=class_str>{children()}</header> }
}

/// `<nav class="sidebar-nav">` slot — the main navigation region.
#[component]
pub fn SidebarNav(
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    let class_str = class
        .map(|c| format!("sidebar-nav {c}"))
        .unwrap_or_else(|| "sidebar-nav".to_string());
    view! { <nav class=class_str>{children()}</nav> }
}

/// `<footer class="sidebar-footer">` slot.
#[component]
pub fn SidebarFooter(
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    let class_str = class
        .map(|c| format!("sidebar-footer {c}"))
        .unwrap_or_else(|| "sidebar-footer".to_string());
    view! { <footer class=class_str>{children()}</footer> }
}

/// The sibling toggle button for a [`Sidebar`].
///
/// Renders next to (NOT inside) the `<aside>`. Carries
/// `data-sidebar-toggle="{target_id}"`, `aria-controls="{target_id}"`, and
/// `aria-expanded` initialized from `default_open`. The WASM controller
/// flips state on click.
#[component]
pub fn SidebarToggle(
    /// The `id` of the [`Sidebar`] this button controls.
    #[prop(into)]
    target_id: Cow<'static, str>,
    /// Whether the controlled sidebar starts expanded. Mirrors the sidebar's
    /// own `default_open` prop so initial markup is in sync.
    #[prop(default = true)]
    default_open: bool,
    /// Extra CSS classes.
    #[prop(optional, into)]
    class: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    let expanded = if default_open { "true" } else { "false" };
    let target = target_id.to_string();
    view! {
        <button
            type="button"
            class=class.map(|c| c.to_string())
            data-sidebar-toggle=target.clone()
            aria-controls=target
            aria-expanded=expanded
        >
            {children()}
        </button>
    }
}
