use crate::props::input::InputProps;

/// Returns the canonical CSS class string for an input.
///
/// Upstream: single `.input` class.
pub fn input(p: &InputProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("input {extra}"),
        _ => "input".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::input::InputProps;

    #[test]
    fn base_class() {
        assert_eq!(input(&InputProps::default()), "input");
    }
}
