use crate::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// Tooltip side — maps to `data-side` attribute on the trigger element.
/// Upstream basecoat implements tooltips purely via CSS using `data-tooltip`
/// and `data-side` attributes — no JS controller needed.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl std::fmt::Display for TooltipSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TooltipSide::Top => "top",
            TooltipSide::Right => "right",
            TooltipSide::Bottom => "bottom",
            TooltipSide::Left => "left",
        };
        f.write_str(s)
    }
}

/// Tooltip — CSS-only via `data-tooltip` and optional `data-side` attributes.
/// No separate container element needed — the tooltip wraps the trigger.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct TooltipProps {
    /// The tooltip text (goes into `data-tooltip`).
    #[prop(into)]
    pub content: Cow<'static, str>,
    #[prop(default)]
    pub side: TooltipSide,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    /// The trigger element (button, link, etc.) rendered inside.
    pub children: Children,
}
