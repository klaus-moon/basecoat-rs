use crate::{AttrMap, BasecoatProps};
use std::borrow::Cow;

/// Input — maps to CSS class `.input`. No variants.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct InputProps {
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    /// HTML `type` attribute (text, email, password, etc.)
    #[prop(optional, into)]
    pub r#type: Option<Cow<'static, str>>,
    #[prop(optional, into)]
    pub placeholder: Option<Cow<'static, str>>,
    #[prop(optional, into)]
    pub value: Option<Cow<'static, str>>,
    #[prop(default)]
    pub disabled: bool,
    #[prop(extend)]
    pub attrs: AttrMap,
}
