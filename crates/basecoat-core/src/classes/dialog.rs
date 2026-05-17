use crate::props::dialog::DialogProps;

/// Returns the canonical CSS class string for a dialog.
///
/// Upstream: `.dialog` class on the `<dialog>` element.
pub fn dialog(p: &DialogProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("dialog {extra}"),
        _ => "dialog".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::dialog::DialogProps;

    #[test]
    fn base_class() {
        assert_eq!(dialog(&DialogProps::default()), "dialog");
    }

    #[test]
    fn with_extra_class() {
        let p = DialogProps {
            class: Some("max-w-lg".into()),
            ..Default::default()
        };
        assert_eq!(dialog(&p), "dialog max-w-lg");
    }
}
