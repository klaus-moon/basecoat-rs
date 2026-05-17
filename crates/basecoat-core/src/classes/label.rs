use crate::props::label::LabelProps;

/// Returns the canonical CSS class string for a label.
///
/// Upstream: single `.label` class.
pub fn label(p: &LabelProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("label {extra}"),
        _ => "label".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::label::LabelProps;

    #[test]
    fn base_class() {
        assert_eq!(label(&LabelProps::default()), "label");
    }
}
