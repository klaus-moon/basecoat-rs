use crate::props::popover::PopoverProps;

/// Returns the canonical CSS class string for a popover root.
///
/// Upstream: `.popover` class on the `<details>` element. Extra classes from
/// `PopoverProps::class` are appended after the base class.
pub fn popover(p: &PopoverProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("popover {extra}"),
        _ => "popover".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::popover::{PopoverPlacement, PopoverProps};

    #[test]
    fn base_class() {
        assert_eq!(popover(&PopoverProps::default()), "popover");
    }

    #[test]
    fn with_extra_class() {
        let p = PopoverProps {
            class: Some("w-72".into()),
            ..Default::default()
        };
        assert_eq!(popover(&p), "popover w-72");
    }

    #[test]
    fn empty_extra_class_collapses_to_base() {
        let p = PopoverProps {
            class: Some("".into()),
            ..Default::default()
        };
        assert_eq!(popover(&p), "popover");
    }

    #[test]
    fn default_placement_is_bottom() {
        let p = PopoverProps::default();
        assert_eq!(p.placement, PopoverPlacement::Bottom);
        assert_eq!(p.placement.as_str(), "bottom");
    }

    #[test]
    fn placement_strings_match_floating_ui_vocabulary() {
        assert_eq!(PopoverPlacement::Top.as_str(), "top");
        assert_eq!(PopoverPlacement::TopStart.as_str(), "top-start");
        assert_eq!(PopoverPlacement::TopEnd.as_str(), "top-end");
        assert_eq!(PopoverPlacement::Bottom.as_str(), "bottom");
        assert_eq!(PopoverPlacement::BottomStart.as_str(), "bottom-start");
        assert_eq!(PopoverPlacement::BottomEnd.as_str(), "bottom-end");
        assert_eq!(PopoverPlacement::Left.as_str(), "left");
        assert_eq!(PopoverPlacement::LeftStart.as_str(), "left-start");
        assert_eq!(PopoverPlacement::LeftEnd.as_str(), "left-end");
        assert_eq!(PopoverPlacement::Right.as_str(), "right");
        assert_eq!(PopoverPlacement::RightStart.as_str(), "right-start");
        assert_eq!(PopoverPlacement::RightEnd.as_str(), "right-end");
    }

    #[test]
    fn builder_default_offset_is_eight_pixels() {
        // The `#[prop(default = 8.0)]` annotation applies only when constructing
        // via the generated builder. `PopoverProps::default()` uses the
        // derived `Default` and yields `0.0` for `f64`.
        let p = PopoverProps::builder()
            .children(crate::Children::empty())
            .build();
        assert_eq!(p.offset_px, 8.0);
    }

    #[test]
    fn builder_default_arrow_is_false() {
        let p = PopoverProps::builder()
            .children(crate::Children::empty())
            .build();
        assert!(!p.arrow);
    }
}
