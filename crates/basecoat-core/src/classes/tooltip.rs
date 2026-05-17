use crate::props::tooltip::TooltipProps;

/// Returns the canonical CSS class string for a tooltip trigger wrapper.
///
/// Upstream basecoat implements tooltips via CSS using `data-tooltip` and
/// `data-side` attributes on the trigger element — no dedicated CSS class.
/// This function returns the extra class if provided, otherwise empty string.
pub fn tooltip(p: &TooltipProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => extra.to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::tooltip::TooltipProps;
    use std::borrow::Cow;

    #[test]
    fn no_class_by_default() {
        let p = TooltipProps {
            content: Cow::Borrowed("Hello"),
            ..Default::default()
        };
        assert_eq!(tooltip(&p), "");
    }
}
