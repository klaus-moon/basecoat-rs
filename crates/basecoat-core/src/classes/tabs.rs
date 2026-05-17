use crate::props::tabs::TabsProps;

/// Returns the canonical CSS class string for a tabs container.
///
/// Upstream: `.tabs` class on the outer `<div>`.
pub fn tabs(p: &TabsProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("tabs {extra}"),
        _ => "tabs".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::tabs::TabsProps;

    #[test]
    fn base_class() {
        assert_eq!(tabs(&TabsProps::default()), "tabs");
    }
}
