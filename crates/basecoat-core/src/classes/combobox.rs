use crate::props::combobox::ComboboxProps;

/// Returns the canonical CSS class string for a combobox wrapper.
///
/// Upstream basecoat styles `input[role=combobox]` through the `.select`
/// layer, so the wrapper `<div>` receives `.select` plus any user-supplied
/// extras.
pub fn combobox(p: &ComboboxProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("select {extra}"),
        _ => "select".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::combobox::{ComboboxOption, ComboboxProps};

    #[test]
    fn base_class() {
        assert_eq!(combobox(&ComboboxProps::default()), "select");
    }

    #[test]
    fn appends_extra_class() {
        let props = ComboboxProps {
            class: Some("w-full".into()),
            ..Default::default()
        };
        assert_eq!(combobox(&props), "select w-full");
    }

    #[test]
    fn ignores_empty_extra_class() {
        let props = ComboboxProps {
            class: Some("".into()),
            ..Default::default()
        };
        assert_eq!(combobox(&props), "select");
    }

    #[test]
    fn option_constructor_round_trips() {
        let opt = ComboboxOption::new("apple", "Apple");
        assert_eq!(opt.value, "apple");
        assert_eq!(opt.label, "Apple");
    }
}
