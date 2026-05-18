use crate::{AttrMap, BasecoatProps, Children, Markup};
use std::borrow::Cow;

/// Sidebar — maps to CSS class `.sidebar`.
///
/// Renders a responsive `<aside>` that behaves as an in-flow column above the
/// `breakpoint_px` viewport width and as an overlay drawer below it. The
/// matching WASM controller (`basecoat-controllers`) reads/writes the
/// expanded/collapsed state to `localStorage` under the key
/// `basecoat:sidebar:{id}` and toggles the `data-state` attribute.
///
/// The `id` is required: it is the key used both for the localStorage entry
/// and for matching the sibling toggle button's `aria-controls`/
/// `data-sidebar-toggle` attribute.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct SidebarProps {
    /// Unique DOM id — required for the controller and localStorage key.
    #[prop(optional, into)]
    pub id: Option<Cow<'static, str>>,
    /// Optional `<header class="sidebar-header">` content rendered at the top.
    #[prop(optional)]
    pub header: Option<Markup>,
    /// Optional `<footer class="sidebar-footer">` content rendered at the bottom.
    #[prop(optional)]
    pub footer: Option<Markup>,
    /// Initial expanded state. Defaults to `true`. The controller reconciles
    /// this against `localStorage` on hydrate.
    #[prop(default = true)]
    pub default_open: bool,
    /// Viewport breakpoint in CSS pixels above which the sidebar is in-flow
    /// rather than an overlay drawer. Defaults to `768.0` (Tailwind `md`).
    #[prop(default = 768.0)]
    pub breakpoint_px: f64,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
