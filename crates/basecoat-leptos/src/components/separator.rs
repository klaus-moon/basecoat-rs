use leptos::prelude::*;

/// Leptos wrapper for the basecoat Separator component.
///
/// Renders `<hr role="separator">`. No CSS class — upstream basecoat uses
/// `role="separator"` for styling.
#[component]
pub fn Separator() -> impl IntoView {
    view! { <hr role="separator" /> }
}
