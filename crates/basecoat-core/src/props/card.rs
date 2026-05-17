use crate::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// Card has a single CSS class `.card` — no variants.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct CardProps {
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
