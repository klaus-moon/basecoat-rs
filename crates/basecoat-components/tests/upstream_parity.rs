use basecoat_components::button;
use basecoat_core::{ButtonProps, ButtonSize, ButtonVariant, Children};

/// Canary test: proves alignment with upstream basecoat's compound class pattern.
/// Upstream: `btn-lg-destructive` (NOT `btn btn-lg btn-destructive`).
#[test]
fn button_lg_destructive_compound_class_and_text() {
    let html = button(
        ButtonProps::builder()
            .variant(ButtonVariant::Destructive)
            .size(ButtonSize::Lg)
            .children(Children::from("Delete"))
            .build(),
    )
    .to_string();

    assert!(
        html.contains("btn-lg-destructive"),
        "Expected class 'btn-lg-destructive' in: {html}"
    );
    assert!(html.contains("Delete"), "Expected text 'Delete' in: {html}");
}
