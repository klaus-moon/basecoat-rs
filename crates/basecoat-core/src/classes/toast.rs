use crate::props::toast::ToastProps;

/// Returns the canonical CSS class string for a toast.
///
/// Upstream: single `.toast` class. The visual category (success/error/info/warning)
/// is communicated via `data-category` attribute, not an additional CSS class.
pub fn toast(p: &ToastProps) -> String {
    match &p.class {
        Some(extra) if !extra.is_empty() => format!("toast {extra}"),
        _ => "toast".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::toast::{ToastCategory, ToastProps};

    #[test]
    fn base_class_success() {
        assert_eq!(toast(&ToastProps::default()), "toast");
    }

    #[test]
    fn base_class_error() {
        let p = ToastProps {
            category: ToastCategory::Error,
            ..Default::default()
        };
        // error is expressed via data-category, not extra CSS class
        assert_eq!(toast(&p), "toast");
    }
}
