use crate::{AttrMap, BasecoatProps};
use std::borrow::Cow;

/// Tabs orientation — upstream uses `aria-orientation` on the tablist `<nav>`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl std::fmt::Display for TabsOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TabsOrientation::Horizontal => f.write_str("horizontal"),
            TabsOrientation::Vertical => f.write_str("vertical"),
        }
    }
}

/// A single tab within a `TabsProps`.
#[derive(Clone, Debug, Default)]
pub struct TabSet {
    /// Tab button content (HTML).
    pub tab: Cow<'static, str>,
    /// Tab panel content (HTML), optional.
    pub panel: Option<Cow<'static, str>>,
}

/// Tabs — maps to CSS class `.tabs`.
#[derive(BasecoatProps, Default, Clone, Debug)]
pub struct TabsProps {
    /// Unique DOM id — required for the WASM controller.
    #[prop(optional, into)]
    pub id: Option<Cow<'static, str>>,
    /// The tab sets (tab buttons + panels).
    #[prop(default)]
    pub tabsets: Vec<TabSet>,
    /// 1-based index of the initially active tab.
    #[prop(default = 1usize)]
    pub default_tab_index: usize,
    #[prop(default)]
    pub orientation: TabsOrientation,
    #[prop(optional, into)]
    pub class: Option<Cow<'static, str>>,
    #[prop(extend)]
    pub attrs: AttrMap,
}
