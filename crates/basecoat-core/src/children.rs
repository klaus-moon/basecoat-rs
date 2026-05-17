use std::borrow::Cow;
use std::fmt;

use crate::Markup;

/// Pre-rendered inner HTML produced by `rsx!`.
///
/// Phase 2c (`rsx!`) will produce `Children` values by recursively rendering
/// inner nodes.  You can also build them directly from strings or `Markup`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Children(pub Cow<'static, str>);

impl Children {
    /// An empty children fragment (allocates nothing).
    pub fn empty() -> Self {
        Children(Cow::Borrowed(""))
    }
}

impl fmt::Display for Children {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Children {
    fn from(s: String) -> Self {
        Children(Cow::Owned(s))
    }
}

impl From<&'static str> for Children {
    fn from(s: &'static str) -> Self {
        Children(Cow::Borrowed(s))
    }
}

impl From<Markup> for Children {
    fn from(m: Markup) -> Self {
        Children(m.0)
    }
}
