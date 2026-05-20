//! SSR smoke tests — verify that components render to sensible HTML bytes.
//!
//! Build with: `cargo test -p basecoat-leptos --features ssr --no-default-features`
//!
//! Uses `RenderHtml::to_html()` (from `tachys::prelude` via `leptos::prelude`),
//! which is the correct SSR rendering method in Leptos 0.8.

#![cfg(feature = "ssr")]

use basecoat_core::{AlertVariant, BadgeVariant, ButtonSize, ButtonVariant};
use basecoat_leptos::*;
use leptos::prelude::*;

#[test]
fn button_default_renders_btn_primary() {
    let html = view! { <Button>"Click me"</Button> }.to_html();
    assert!(
        html.contains("<button"),
        "expected <button> tag, got: {html}"
    );
    assert!(
        html.contains("btn-primary"),
        "expected btn-primary class, got: {html}"
    );
    assert!(html.contains("Click me"), "expected children, got: {html}");
}

#[test]
fn button_outline_sm() {
    let html = view! {
        <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
            "Save"
        </Button>
    }
    .to_html();
    assert!(
        html.contains("btn-sm-outline"),
        "expected btn-sm-outline, got: {html}"
    );
}

#[test]
fn button_lg_destructive() {
    let html = view! {
        <Button variant=ButtonVariant::Destructive size=ButtonSize::Lg>
            "Delete"
        </Button>
    }
    .to_html();
    assert!(
        html.contains("btn-lg-destructive"),
        "expected btn-lg-destructive, got: {html}"
    );
}

#[test]
fn badge_default_renders_badge() {
    let html = view! { <Badge>"New"</Badge> }.to_html();
    assert!(html.contains("<span"), "expected <span> tag, got: {html}");
    // BadgeVariant::Default renders as "badge" (no suffix)
    assert!(html.contains("badge"), "expected badge class, got: {html}");
}

#[test]
fn badge_secondary() {
    let html = view! { <Badge variant=BadgeVariant::Secondary>"Beta"</Badge> }.to_html();
    assert!(
        html.contains("badge-secondary"),
        "expected badge-secondary, got: {html}"
    );
}

#[test]
fn alert_default() {
    let html = view! { <Alert>"Something happened"</Alert> }.to_html();
    assert!(
        html.contains(r#"role="alert""#),
        "expected role=alert, got: {html}"
    );
    assert!(html.contains("alert"), "expected alert class, got: {html}");
}

#[test]
fn alert_destructive() {
    let html = view! { <Alert variant=AlertVariant::Destructive>"Error!"</Alert> }.to_html();
    assert!(
        html.contains("alert-destructive"),
        "expected alert-destructive, got: {html}"
    );
}

#[test]
fn card_renders_div_with_card_class() {
    let html = view! { <Card>"Card content"</Card> }.to_html();
    assert!(html.contains("<div"), "expected <div>, got: {html}");
    assert!(
        html.contains("\"card\""),
        "expected class=\"card\", got: {html}"
    );
}

#[test]
fn input_renders_with_input_class() {
    let html = view! { <Input /> }.to_html();
    assert!(html.contains("<input"), "expected <input>, got: {html}");
    assert!(html.contains("input"), "expected input class, got: {html}");
}

#[test]
fn label_renders_with_label_class() {
    let html = view! { <Label>"Email"</Label> }.to_html();
    assert!(html.contains("<label"), "expected <label>, got: {html}");
    assert!(html.contains("label"), "expected label class, got: {html}");
}

#[test]
fn textarea_renders_with_textarea_class() {
    let html = view! { <Textarea /> }.to_html();
    assert!(
        html.contains("<textarea"),
        "expected <textarea>, got: {html}"
    );
    assert!(
        html.contains("textarea"),
        "expected textarea class, got: {html}"
    );
}

#[test]
fn separator_renders_hr_with_role() {
    let html = view! { <Separator /> }.to_html();
    assert!(html.contains("<hr"), "expected <hr>, got: {html}");
    assert!(
        html.contains(r#"role="separator""#),
        "expected role=separator, got: {html}"
    );
}

#[test]
fn dialog_emits_hydration_markers() {
    let html = view! {
        <Dialog id="my-dialog" title="Hello">
            "Dialog content"
        </Dialog>
    }
    .to_html();
    assert!(
        html.contains("<dialog"),
        "expected <dialog> element, got: {html}"
    );
    assert!(
        html.contains(r#"data-basecoat-hydrate="dialog""#),
        "expected data-basecoat-hydrate=dialog, got: {html}"
    );
    assert!(
        html.contains(r#"data-basecoat-version="0.1""#),
        "expected data-basecoat-version=0.1, got: {html}"
    );
    assert!(
        html.contains("dialog"),
        "expected dialog class, got: {html}"
    );
}

#[test]
fn tabs_emits_hydration_markers() {
    let html = view! {
        <Tabs id="my-tabs">
            <TabsList>
                <TabsTab controls="panel-1" selected=true>
                    "Tab 1"
                </TabsTab>
                <TabsTab controls="panel-2">"Tab 2"</TabsTab>
            </TabsList>
            <TabsPanel id="panel-1" selected=true>
                "Content 1"
            </TabsPanel>
            <TabsPanel id="panel-2">"Content 2"</TabsPanel>
        </Tabs>
    }
    .to_html();
    assert!(html.contains("tabs"), "expected tabs class, got: {html}");
    assert!(
        html.contains(r#"data-basecoat-hydrate="tabs""#),
        "expected data-basecoat-hydrate=tabs, got: {html}"
    );
    assert!(
        html.contains(r#"data-basecoat-version="0.1""#),
        "expected data-basecoat-version=0.1, got: {html}"
    );
}

#[test]
fn toast_emits_hydration_markers() {
    let html = view! {
        <Toaster>
            <Toast title="Done" description="Saved successfully." />
        </Toaster>
    }
    .to_html();
    assert!(
        html.contains(r#"data-basecoat-hydrate="toast""#),
        "expected data-basecoat-hydrate=toast, got: {html}"
    );
    assert!(html.contains("toast"), "expected toast class, got: {html}");
    assert!(
        html.contains(r#"data-category="success""#),
        "expected data-category=success, got: {html}"
    );
}

#[test]
fn tooltip_renders_data_tooltip() {
    let html = view! {
        <Tooltip content="Helpful tip">
            <button type="button">"Hover me"</button>
        </Tooltip>
    }
    .to_html();
    assert!(
        html.contains(r#"data-tooltip="Helpful tip""#),
        "expected data-tooltip attribute, got: {html}"
    );
    assert!(
        html.contains("<span"),
        "expected wrapping span, got: {html}"
    );
}

/// Confirm class-string parity between this crate and basecoat_core::classes directly.
#[test]
fn class_string_parity_with_core() {
    use basecoat_core::{classes, ButtonProps};

    let core_class = classes::button(&ButtonProps {
        variant: ButtonVariant::Outline,
        size: ButtonSize::Sm,
        ..Default::default()
    });

    let html = view! {
        <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
            "x"
        </Button>
    }
    .to_html();

    assert!(
        html.contains(&core_class),
        "Leptos class `{core_class}` not found in rendered HTML: {html}"
    );
}
