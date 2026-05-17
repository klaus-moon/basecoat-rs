/// Build script for basecoat-components.
///
/// Generates `OUT_DIR/basecoat-classes.txt`: a sorted, deduplicated list of every
/// Tailwind class that any basecoat component can emit.  This is the safelist
/// that consumers add to their Tailwind config so the CSS scanner picks them up.
use basecoat_core::classes;
use basecoat_core::{
    AlertProps, AlertVariant, BadgeProps, BadgeVariant, ButtonProps, ButtonSize, ButtonVariant,
    CardProps, DialogProps, InputProps, LabelProps, SeparatorProps, TabsProps, TextareaProps,
    ToastProps, TooltipProps,
};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src");

    let mut set: BTreeSet<String> = BTreeSet::new();

    // ── Button ───────────────────────────────────────────────────────────────
    for variant in [
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Outline,
        ButtonVariant::Ghost,
        ButtonVariant::Link,
        ButtonVariant::Destructive,
    ] {
        for size in [
            ButtonSize::Default,
            ButtonSize::Sm,
            ButtonSize::Lg,
            ButtonSize::Icon,
        ] {
            let p = ButtonProps {
                variant: variant.clone(),
                size: size.clone(),
                ..Default::default()
            };
            for cls in classes::button(&p).split_whitespace() {
                set.insert(cls.to_string());
            }
        }
    }

    // ── Badge ────────────────────────────────────────────────────────────────
    for variant in [
        BadgeVariant::Default,
        BadgeVariant::Secondary,
        BadgeVariant::Destructive,
        BadgeVariant::Outline,
    ] {
        let p = BadgeProps {
            variant,
            ..Default::default()
        };
        for cls in classes::badge(&p).split_whitespace() {
            set.insert(cls.to_string());
        }
    }

    // ── Alert ────────────────────────────────────────────────────────────────
    for variant in [AlertVariant::Default, AlertVariant::Destructive] {
        let p = AlertProps {
            variant,
            ..Default::default()
        };
        for cls in classes::alert(&p).split_whitespace() {
            set.insert(cls.to_string());
        }
    }

    // ── Static single-class components ──────────────────────────────────────
    for cls in classes::card(&CardProps::default()).split_whitespace() {
        set.insert(cls.to_string());
    }
    for cls in classes::input(&InputProps::default()).split_whitespace() {
        set.insert(cls.to_string());
    }
    for cls in classes::label(&LabelProps::default()).split_whitespace() {
        set.insert(cls.to_string());
    }
    for cls in classes::textarea(&TextareaProps::default()).split_whitespace() {
        set.insert(cls.to_string());
    }
    for cls in classes::dialog(&DialogProps::default()).split_whitespace() {
        set.insert(cls.to_string());
    }
    for cls in classes::tabs(&TabsProps::default()).split_whitespace() {
        set.insert(cls.to_string());
    }
    for cls in classes::toast(&ToastProps::default()).split_whitespace() {
        set.insert(cls.to_string());
    }
    // toast always emits "toast"; category variants use data-category, not classes
    set.insert("toast".to_string());
    set.insert("toaster".to_string());

    // separator returns "" — no CSS class, just role attribute
    let _ = classes::separator(&SeparatorProps::default());

    // tooltip returns "" by default; only user-provided extra class
    let _ = classes::tooltip(&TooltipProps {
        content: std::borrow::Cow::Borrowed(""),
        ..Default::default()
    });

    // Write output
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("basecoat-classes.txt");
    let content: String = set.into_iter().collect::<Vec<_>>().join("\n") + "\n";
    fs::write(&dest, content).unwrap();
}
