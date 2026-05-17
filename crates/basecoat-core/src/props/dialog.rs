use crate::{AttrMap, BasecoatProps, Children, Markup};
use std::borrow::Cow;

/// Dialog — maps to CSS class `.dialog`.
///
/// Upstream basecoat uses the native `<dialog>` element with a JS controller
/// for open/close. The `id` is required for the controller to find the element.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct DialogProps {
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    pub id: Option<Cow<'static, str>>,
    /// Optional trigger button content.
    #[prop(optional)]
    pub trigger: Option<Markup>,
    /// Dialog title displayed in the `<header>`.
    #[prop(optional, into)]
    pub title: Option<Cow<'static, str>>,
    /// Dialog description displayed below the title.
    #[prop(optional, into)]
    pub description: Option<Cow<'static, str>>,
    /// Footer HTML content.
    #[prop(optional)]
    pub footer: Option<Markup>,
    /// Whether to render a close button (default true).
    #[prop(default = true)]
    pub close_button: bool,
    /// Whether clicking the overlay closes the dialog (default true).
    #[prop(default = true)]
    pub close_on_overlay_click: bool,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
