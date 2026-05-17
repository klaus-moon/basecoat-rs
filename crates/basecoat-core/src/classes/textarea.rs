use crate::props::textarea::TextareaProps;

/// Returns the canonical CSS class string for a textarea.
///
/// Upstream: single `.textarea` class.
pub fn textarea(p: &TextareaProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("textarea {extra}"),
        _ => "textarea".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::textarea::TextareaProps;

    #[test]
    fn base_class() {
        assert_eq!(textarea(&TextareaProps::default()), "textarea");
    }
}
