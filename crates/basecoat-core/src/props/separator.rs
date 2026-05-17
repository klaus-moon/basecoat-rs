use crate::{AttrMap, BasecoatProps};

/// Separator — rendered as `<hr role="separator">`. No CSS class, no variants.
/// Upstream basecoat uses `role="separator"` with CSS that targets that attribute.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct SeparatorProps {
    #[prop(extend)]
    pub attrs: AttrMap,
}
