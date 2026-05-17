use crate::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// Label — maps to CSS class `.label`. No variants.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct LabelProps {
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    /// `for` attribute linking to an input id.
    #[prop(optional, into)]
    pub r#for: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
