use basecoat_core::{AttrMap, BasecoatProps, Children};
use std::borrow::Cow;

/// Generic sub-component props reused across compound components
/// (card_header, card_title, dialog_content, tabs_list, etc.)
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct SubProps {
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
    pub children: Children,
}
