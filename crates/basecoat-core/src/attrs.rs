use std::borrow::Cow;

/// Escape an attribute value: `&`, `<`, `>`, `"`, `'`.
/// Only allocates when escaping is actually needed.
pub fn escape_attr(s: &str) -> Cow<'_, str> {
    // Fast path: scan for any char that needs escaping.
    let needs_escape = s.chars().any(|c| matches!(c, '&' | '<' | '>' | '"' | '\''));
    if !needs_escape {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    Cow::Owned(out)
}

/// An ordered list of HTML attribute key-value pairs.
///
/// Keys and values are stored as `Cow<'static, str>` to avoid allocations when
/// attribute names and values are known at compile time (string literals).
///
/// Duplicate keys are allowed (the last one wins in most browsers, but we
/// preserve insertion order for deterministic output and snapshot tests).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AttrMap(pub Vec<(Cow<'static, str>, Cow<'static, str>)>);

impl AttrMap {
    /// Create an empty `AttrMap`.
    pub fn new() -> Self {
        AttrMap(Vec::new())
    }

    /// Append a key-value pair.
    pub fn push(&mut self, key: impl Into<Cow<'static, str>>, val: impl Into<Cow<'static, str>>) {
        self.0.push((key.into(), val.into()));
    }

    /// Extend from any iterator of (key, value) pairs.
    pub fn extend_from<I, K, V>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        for (k, v) in iter {
            self.0.push((k.into(), v.into()));
        }
    }

    /// Iterate over `(&str, &str)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }

    /// Render as an HTML attribute string.
    ///
    /// Returns `" key=\"value\" key2=\"value2\""` (leading space when non-empty)
    /// or `""` when empty.  Values are HTML-attribute-escaped.
    pub fn render(&self) -> String {
        if self.0.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        for (k, v) in &self.0 {
            out.push(' ');
            out.push_str(k);
            out.push_str("=\"");
            out.push_str(&escape_attr(v));
            out.push('"');
        }
        out
    }
}

impl<K, V> FromIterator<(K, V)> for AttrMap
where
    K: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = AttrMap::new();
        for (k, v) in iter {
            map.0.push((k.into(), v.into()));
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_renders_empty_string() {
        let m = AttrMap::new();
        assert_eq!(m.render(), "");
    }

    #[test]
    fn single_attr_renders_with_leading_space() {
        let mut m = AttrMap::new();
        m.push("id", "foo");
        assert_eq!(m.render(), r#" id="foo""#);
    }

    #[test]
    fn escapes_dangerous_chars_in_values() {
        let mut m = AttrMap::new();
        m.push("data-x", r#"a"b&c<d>e'f"#);
        assert_eq!(m.render(), r#" data-x="a&quot;b&amp;c&lt;d&gt;e&#39;f""#);
    }

    #[test]
    fn escape_attr_no_alloc_on_safe_string() {
        let s = "hello world";
        let escaped = escape_attr(s);
        assert!(matches!(escaped, Cow::Borrowed(_)));
    }
}
