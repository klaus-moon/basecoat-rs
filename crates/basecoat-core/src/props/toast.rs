use crate::{AttrMap, BasecoatProps};
use std::borrow::Cow;

/// Toast category — maps to `data-category` attribute.
/// Determines icon and ARIA role in upstream basecoat.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ToastCategory {
    #[default]
    Success,
    Error,
    Info,
    Warning,
}

impl std::fmt::Display for ToastCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ToastCategory::Success => "success",
            ToastCategory::Error => "error",
            ToastCategory::Info => "info",
            ToastCategory::Warning => "warning",
        };
        f.write_str(s)
    }
}

/// Props for a single toast message.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct ToastProps {
    #[prop(default)]
    pub category: ToastCategory,
    #[prop(optional, into)]
    pub title: Option<Cow<'static, str>>,
    #[prop(optional, into)]
    pub description: Option<Cow<'static, str>>,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
}

/// Props for the toaster container that holds multiple toasts.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct ToasterProps {
    #[prop(default = Cow::Borrowed("toaster"))]
    pub id: Cow<'static, str>,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
}
