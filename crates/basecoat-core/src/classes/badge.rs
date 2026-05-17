use crate::props::badge::{BadgeProps, BadgeVariant};

/// Returns the canonical CSS class string for a badge.
///
/// Upstream: `.badge`, `.badge-secondary`, `.badge-destructive`, `.badge-outline`.
pub fn badge(p: &BadgeProps) -> String {
    let base = match p.variant {
        BadgeVariant::Default => "badge",
        BadgeVariant::Secondary => "badge-secondary",
        BadgeVariant::Destructive => "badge-destructive",
        BadgeVariant::Outline => "badge-outline",
    };

    match &p.class {
        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::badge::{BadgeProps, BadgeVariant};

    #[test]
    fn default_variant() {
        assert_eq!(badge(&BadgeProps::default()), "badge");
    }

    #[test]
    fn secondary_variant() {
        let p = BadgeProps {
            variant: BadgeVariant::Secondary,
            ..Default::default()
        };
        assert_eq!(badge(&p), "badge-secondary");
    }

    #[test]
    fn destructive_variant() {
        let p = BadgeProps {
            variant: BadgeVariant::Destructive,
            ..Default::default()
        };
        assert_eq!(badge(&p), "badge-destructive");
    }

    #[test]
    fn outline_variant() {
        let p = BadgeProps {
            variant: BadgeVariant::Outline,
            ..Default::default()
        };
        assert_eq!(badge(&p), "badge-outline");
    }
}
