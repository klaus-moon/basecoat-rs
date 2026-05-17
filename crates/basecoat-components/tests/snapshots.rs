use basecoat_components::*;
use basecoat_core::props::tabs::TabSet;
use basecoat_core::*;

// ── Button ───────────────────────────────────────────────────────────────────

#[test]
fn snapshot_button_default() {
    let html = button(ButtonProps {
        children: Children::from("Click me"),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

#[test]
fn snapshot_button_lg_destructive() {
    let html = button(ButtonProps {
        variant: ButtonVariant::Destructive,
        size: ButtonSize::Lg,
        children: Children::from("Delete"),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Input ────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_input_email() {
    let html = input(InputProps {
        r#type: Some("email".into()),
        placeholder: Some("you@example.com".into()),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Label ────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_label() {
    let html = label(LabelProps {
        r#for: Some("email".into()),
        children: Children::from("Email address"),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Textarea ─────────────────────────────────────────────────────────────────

#[test]
fn snapshot_textarea() {
    let html = textarea(TextareaProps {
        placeholder: Some("Type your message here".into()),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Card ─────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_card() {
    let html = card(CardProps {
        children: Children::from("<header><h2>Title</h2></header><section>Body</section>"),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Badge ────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_badge_destructive() {
    let html = badge(BadgeProps {
        variant: BadgeVariant::Destructive,
        children: Children::from("Error"),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Alert ────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_alert_destructive() {
    let html = alert(AlertProps {
        variant: AlertVariant::Destructive,
        children: Children::from("<h2>Error</h2><section>Something went wrong.</section>"),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Separator ────────────────────────────────────────────────────────────────

#[test]
fn snapshot_separator() {
    let html = separator(SeparatorProps::default());
    insta::assert_snapshot!(html.to_string());
}

// ── Dialog ───────────────────────────────────────────────────────────────────

#[test]
fn snapshot_dialog() {
    let html = dialog(DialogProps {
        id: Some("test-dialog".into()),
        trigger: Some(Markup::from_static("Open")),
        title: Some("Edit Profile".into()),
        description: Some("Make changes here.".into()),
        close_button: true,
        close_on_overlay_click: true,
        children: Children::from("<p>Dialog body content</p>"),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Tabs ─────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_tabs() {
    let html = tabs(TabsProps {
        id: Some("demo-tabs".into()),
        tabsets: vec![
            TabSet {
                tab: "Account".into(),
                panel: Some("Account panel content".into()),
            },
            TabSet {
                tab: "Password".into(),
                panel: Some("Password panel content".into()),
            },
        ],
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Toast ────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_toast_success() {
    let html = toast(ToastProps {
        category: ToastCategory::Success,
        title: Some("Saved!".into()),
        description: Some("Your changes have been saved.".into()),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}

// ── Tooltip ──────────────────────────────────────────────────────────────────

#[test]
fn snapshot_tooltip() {
    let html = tooltip(TooltipProps {
        content: "Helpful hint".into(),
        children: Children::from(r#"<button type="button" class="btn-outline">Hover me</button>"#),
        ..Default::default()
    });
    insta::assert_snapshot!(html.to_string());
}
