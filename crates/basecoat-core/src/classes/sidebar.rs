use crate::props::sidebar::SidebarProps;

/// Returns the canonical CSS class string for a sidebar.
///
/// Upstream: `.sidebar` class on the `<aside>` element.
pub fn sidebar(p: &SidebarProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("sidebar {extra}"),
        _ => "sidebar".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::sidebar::SidebarProps;

    #[test]
    fn base_class() {
        assert_eq!(sidebar(&SidebarProps::default()), "sidebar");
    }

    #[test]
    fn with_extra_class() {
        let p = SidebarProps {
            class: Some("border-r".into()),
            ..Default::default()
        };
        assert_eq!(sidebar(&p), "sidebar border-r");
    }

    #[test]
    fn empty_extra_class_is_ignored() {
        let p = SidebarProps {
            class: Some("".into()),
            ..Default::default()
        };
        assert_eq!(sidebar(&p), "sidebar");
    }

    #[test]
    fn class_unchanged_regardless_of_state_props() {
        // default_open / breakpoint_px do NOT influence the class string —
        // they only affect rendered attributes / controller behavior.
        let collapsed = SidebarProps {
            default_open: false,
            breakpoint_px: 1024.0,
            ..Default::default()
        };
        assert_eq!(sidebar(&collapsed), "sidebar");
        let expanded = SidebarProps {
            default_open: true,
            breakpoint_px: 768.0,
            class: Some("w-72".into()),
            ..Default::default()
        };
        assert_eq!(sidebar(&expanded), "sidebar w-72");
    }
}
