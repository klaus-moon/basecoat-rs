use crate::props::select::SelectProps;

/// Returns the canonical CSS class string for a select.
///
/// Upstream: `.select` class on the outer wrapper element (a non-`<select>`
/// container per `*:not(select).select` in basecoat.css).
pub fn select(p: &SelectProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("select {extra}"),
        _ => "select".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::select::{SelectOption, SelectProps};

    #[test]
    fn base_class() {
        assert_eq!(select(&SelectProps::default()), "select");
    }

    #[test]
    fn with_extra_class() {
        let p = SelectProps {
            class: Some("w-64".into()),
            ..Default::default()
        };
        assert_eq!(select(&p), "select w-64");
    }

    #[test]
    fn empty_extra_class_is_ignored() {
        let p = SelectProps {
            class: Some("".into()),
            ..Default::default()
        };
        assert_eq!(select(&p), "select");
    }

    #[test]
    fn options_constructor_is_enabled_by_default() {
        let o = SelectOption::new("a", "Apple");
        assert_eq!(o.value, "a");
        assert_eq!(o.label, "Apple");
        assert!(!o.disabled);
    }

    #[test]
    fn options_disabled_builder() {
        let o = SelectOption::new("b", "Banana").disabled();
        assert!(o.disabled);
    }
}
