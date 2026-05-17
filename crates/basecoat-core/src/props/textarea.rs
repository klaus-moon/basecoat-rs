use crate::{AttrMap, BasecoatProps};
use std::borrow::Cow;

/// Textarea — maps to CSS class `.textarea`. No variants.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct TextareaProps {
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(optional, into)]
    pub placeholder: Option<Cow<'static, str>>,
    #[prop(default)]
    pub disabled: bool,
    #[prop(extend)]
    pub attrs: AttrMap,
}
