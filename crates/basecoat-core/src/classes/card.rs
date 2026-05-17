use crate::props::card::CardProps;

/// Returns the canonical CSS class string for a card.
///
/// Upstream: single `.card` class, no variants.
pub fn card(p: &CardProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("card {extra}"),
        _ => "card".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::card::CardProps;

    #[test]
    fn base_class() {
        assert_eq!(card(&CardProps::default()), "card");
    }

    #[test]
    fn with_extra_class() {
        let p = CardProps {
            class: Some("mt-4".into()),
            ..Default::default()
        };
        assert_eq!(card(&p), "card mt-4");
    }
}
