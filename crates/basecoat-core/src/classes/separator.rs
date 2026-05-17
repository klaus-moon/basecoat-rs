use crate::props::separator::SeparatorProps;

/// Returns the canonical CSS class string for a separator.
///
/// Upstream basecoat uses `role="separator"` on `<hr>` — no CSS class needed.
/// This function always returns an empty string; the `role` attribute is
/// handled by the component renderer, not the class function.
pub fn separator(_p: &SeparatorProps) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::separator::SeparatorProps;

    #[test]
    fn no_class() {
        assert_eq!(separator(&SeparatorProps::default()), "");
    }
}
