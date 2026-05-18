use crate::{AttrMap, BasecoatProps, Children, Markup};
use std::borrow::Cow;

/// Popover placement — passed to the floating-ui controller as the
/// `placement` argument.
///
/// The string form matches [floating-ui's placement vocabulary](https://floating-ui.com/docs/computePosition#placement)
/// so the WASM controller can forward it verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PopoverPlacement {
    Top,
    TopStart,
    TopEnd,
    #[default]
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

impl PopoverPlacement {
    /// Returns the floating-ui placement token (e.g. `"bottom-start"`).
    pub fn as_str(&self) -> &'static str {
        match self {
            PopoverPlacement::Top => "top",
            PopoverPlacement::TopStart => "top-start",
            PopoverPlacement::TopEnd => "top-end",
            PopoverPlacement::Bottom => "bottom",
            PopoverPlacement::BottomStart => "bottom-start",
            PopoverPlacement::BottomEnd => "bottom-end",
            PopoverPlacement::Left => "left",
            PopoverPlacement::LeftStart => "left-start",
            PopoverPlacement::LeftEnd => "left-end",
            PopoverPlacement::Right => "right",
            PopoverPlacement::RightStart => "right-start",
            PopoverPlacement::RightEnd => "right-end",
        }
    }
}

impl std::fmt::Display for PopoverPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Popover — maps to CSS class `.popover` on a `<details>` element.
///
/// Upstream basecoat uses a `<details>` root containing a `<summary>` trigger
/// and a `<div role="dialog">` content panel. The WASM controller wires
/// floating-ui positioning, click-outside dismissal, and aria-expanded sync
/// onto that markup.
///
/// The `id` is required by the WASM controller so trigger/content pairs can be
/// looked up unambiguously and so accessible-name attributes (`aria-controls`,
/// `aria-labelledby`) can target the right elements.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct PopoverProps {
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    pub id: Option<Cow<'static, str>>,
    /// Trigger content placed inside the `<summary>` element.
    #[prop(optional)]
    pub trigger: Option<Markup>,
    /// Floating-ui placement (default `bottom`).
    #[prop(default)]
    pub placement: PopoverPlacement,
    /// Distance in pixels between the trigger edge and the content panel.
    #[prop(default = 8.0)]
    pub offset_px: f64,
    /// Whether to render a `<div data-popover-arrow>` element inside the
    /// content. The visual arrow is CSS-positioned in v0.2 — see crate docs.
    #[prop(default = false)]
    pub arrow: bool,
    /// Extra CSS classes appended after the `popover` class.
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    /// Popover content (rendered inside the `<div role="dialog">`).
    pub children: Children,
}
