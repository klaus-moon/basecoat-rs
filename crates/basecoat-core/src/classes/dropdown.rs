use crate::props::dropdown::DropdownProps;

/// Returns the canonical CSS class string for a dropdown root element.
///
/// Upstream: `.dropdown-menu` class on the `<details>` element.
pub fn dropdown(p: &DropdownProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("dropdown-menu {extra}"),
        _ => "dropdown-menu".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::dropdown::DropdownProps;

    #[test]
    fn base_class() {
        assert_eq!(dropdown(&DropdownProps::default()), "dropdown-menu");
    }

    #[test]
    fn with_extra_class() {
        let p = DropdownProps {
            class: Some("w-56".into()),
            ..Default::default()
        };
        assert_eq!(dropdown(&p), "dropdown-menu w-56");
    }

    #[test]
    fn empty_extra_class_is_ignored() {
        let p = DropdownProps {
            class: Some("".into()),
            ..Default::default()
        };
        assert_eq!(dropdown(&p), "dropdown-menu");
    }
}
