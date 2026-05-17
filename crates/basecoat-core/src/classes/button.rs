use crate::props::button::{ButtonProps, ButtonSize, ButtonVariant};

/// Returns the canonical CSS class string for a button.
///
/// Upstream basecoat uses compound classes: `btn-{size}-{variant}` for non-default
/// sizes, and `btn-{variant}` for default (no size prefix). Examples:
/// - Default + Primary   → `"btn-primary"`
/// - Sm + Outline        → `"btn-sm-outline"`
/// - Lg + Destructive    → `"btn-lg-destructive"`
/// - Icon + Ghost        → `"btn-icon-ghost"`
pub fn button(p: &ButtonProps) -> String {
    let variant_str = match p.variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Outline => "outline",
        ButtonVariant::Ghost => "ghost",
        ButtonVariant::Link => "link",
        ButtonVariant::Destructive => "destructive",
    };

    let base = match p.size {
        ButtonSize::Default => format!("btn-{variant_str}"),
        ButtonSize::Sm => format!("btn-sm-{variant_str}"),
        ButtonSize::Lg => format!("btn-lg-{variant_str}"),
        ButtonSize::Icon => format!("btn-icon-{variant_str}"),
    };

    match &p.class {
        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::button::{ButtonProps, ButtonSize, ButtonVariant};

    #[test]
    fn default_variant_and_size() {
        let p = ButtonProps::default();
        assert_eq!(button(&p), "btn-primary");
    }

    #[test]
    fn outline_default_size() {
        let p = ButtonProps {
            variant: ButtonVariant::Outline,
            ..Default::default()
        };
        assert_eq!(button(&p), "btn-outline");
    }

    #[test]
    fn sm_outline() {
        let p = ButtonProps {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Sm,
            ..Default::default()
        };
        assert_eq!(button(&p), "btn-sm-outline");
    }

    #[test]
    fn lg_destructive() {
        let p = ButtonProps {
            variant: ButtonVariant::Destructive,
            size: ButtonSize::Lg,
            ..Default::default()
        };
        assert_eq!(button(&p), "btn-lg-destructive");
    }

    #[test]
    fn icon_ghost() {
        let p = ButtonProps {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            ..Default::default()
        };
        assert_eq!(button(&p), "btn-icon-ghost");
    }

    #[test]
    fn extra_class_appended() {
        let p = ButtonProps {
            variant: ButtonVariant::Secondary,
            class: Some("w-full".into()),
            ..Default::default()
        };
        assert_eq!(button(&p), "btn-secondary w-full");
    }

    #[test]
    fn sm_secondary() {
        let p = ButtonProps {
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Sm,
            ..Default::default()
        };
        assert_eq!(button(&p), "btn-sm-secondary");
    }

    #[test]
    fn lg_primary() {
        let p = ButtonProps {
            size: ButtonSize::Lg,
            ..Default::default()
        };
        assert_eq!(button(&p), "btn-lg-primary");
    }
}
