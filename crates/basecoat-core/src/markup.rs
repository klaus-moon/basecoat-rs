use std::borrow::Cow;
use std::fmt;

/// Pre-escaped HTML fragment. Do **not** double-escape when rendering.
///
/// Construct with `Markup::from_static` (zero-alloc for string literals) or
/// `Markup::from(String)` for dynamically-built HTML.  Never pass user
/// controlled strings directly — escape them first with `crate::attrs::escape_attr`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Markup(pub Cow<'static, str>);

impl Markup {
    /// An empty markup fragment (allocates nothing).
    pub fn empty() -> Self {
        Markup(Cow::Borrowed(""))
    }

    /// Borrow a `'static` string literal — zero allocation.
    pub fn from_static(s: &'static str) -> Self {
        Markup(Cow::Borrowed(s))
    }
}

impl fmt::Display for Markup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Markup {
    fn from(s: String) -> Self {
        Markup(Cow::Owned(s))
    }
}

impl From<&'static str> for Markup {
    fn from(s: &'static str) -> Self {
        Markup(Cow::Borrowed(s))
    }
}
