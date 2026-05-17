use crate::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// Alert variant — maps to `.alert` or `.alert-destructive`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

impl std::fmt::Display for AlertVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AlertVariant::Default => "alert",
            AlertVariant::Destructive => "alert-destructive",
        };
        f.write_str(s)
    }
}

#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct AlertProps {
    #[prop(default)]
    pub variant: AlertVariant,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
