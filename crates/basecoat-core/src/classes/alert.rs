use crate::props::alert::{AlertProps, AlertVariant};

/// Returns the canonical CSS class string for an alert.
///
/// Upstream: `.alert` or `.alert-destructive`.
pub fn alert(p: &AlertProps) -> String {
    let base = match p.variant {
        AlertVariant::Default => "alert",
        AlertVariant::Destructive => "alert-destructive",
    };

    match &p.class {
        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::alert::{AlertProps, AlertVariant};

    #[test]
    fn default_variant() {
        assert_eq!(alert(&AlertProps::default()), "alert");
    }

    #[test]
    fn destructive_variant() {
        let p = AlertProps {
            variant: AlertVariant::Destructive,
            ..Default::default()
        };
        assert_eq!(alert(&p), "alert-destructive");
    }
}
