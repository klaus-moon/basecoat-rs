use crate::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// Button visual variant — maps to upstream CSS class prefix.
///
/// The upstream class pattern is `btn-{size}-{variant}` (compound).
/// Default size + Default variant → `btn-primary`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// `btn-primary` (default)
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Link,
    Destructive,
}

impl std::fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Link => "link",
            ButtonVariant::Destructive => "destructive",
        };
        f.write_str(s)
    }
}

/// Button size variant.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// Default size — no size prefix in CSS class.
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

impl std::fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ButtonSize::Default => "",
            ButtonSize::Sm => "sm",
            ButtonSize::Lg => "lg",
            ButtonSize::Icon => "icon",
        };
        f.write_str(s)
    }
}

#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct ButtonProps {
    #[prop(default)]
    pub variant: ButtonVariant,
    #[prop(default)]
    pub size: ButtonSize,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
