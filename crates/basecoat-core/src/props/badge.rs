use crate::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// Badge visual variant — maps to `.badge`, `.badge-secondary`, etc.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    /// `.badge` / `.badge-primary`
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

impl std::fmt::Display for BadgeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BadgeVariant::Default => "badge",
            BadgeVariant::Secondary => "badge-secondary",
            BadgeVariant::Destructive => "badge-destructive",
            BadgeVariant::Outline => "badge-outline",
        };
        f.write_str(s)
    }
}

#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct BadgeProps {
    #[prop(default)]
    pub variant: BadgeVariant,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
