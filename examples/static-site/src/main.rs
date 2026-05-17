//! Static-site example — pure `rsx!` to HTML file, no web framework.
//!
//! Run with:
//! ```sh
//! cargo run -p static-site
//! ```
//!
//! Produces `examples/static-site/dist/index.html`.

// rsx! expression blocks `{expr}` trigger unused_braces when the expression is
// a single identifier. The braces are required by the rsx! grammar.
#![allow(unused_braces)]

use std::borrow::Cow;

use basecoat_rs::components::{
    SubProps, card, card_content, card_description, card_footer, card_header, card_title, tabs,
};
use basecoat_rs::{BadgeVariant, ButtonVariant, CardProps, Children, TabSet, TabsProps, rsx};

fn build_card() -> basecoat_rs::Markup {
    let title = card_title(
        SubProps::builder()
            .children(Children::from("Example Card".to_owned()))
            .build(),
    );
    let desc = card_description(
        SubProps::builder()
            .children(Children::from("Rendered server-side via rsx!.".to_owned()))
            .build(),
    );
    let header = card_header(
        SubProps::builder()
            .children(Children::from(format!("{title}{desc}")))
            .build(),
    );
    let content = card_content(
        SubProps::builder()
            .children(Children::from(
                "<p>Card body content goes here.</p>".to_owned(),
            ))
            .build(),
    );
    let footer = card_footer(
        SubProps::builder()
            .children(Children::from(
                "<button class=\"btn-primary\">Confirm</button>".to_owned(),
            ))
            .build(),
    );

    card(
        CardProps::builder()
            .children(Children::from(format!("{header}{content}{footer}")))
            .build(),
    )
}

fn build_tabs() -> basecoat_rs::Markup {
    let tabsets = vec![
        TabSet {
            tab: Cow::Borrowed("Overview"),
            panel: Some(Cow::Borrowed("<p>Overview panel content.</p>")),
        },
        TabSet {
            tab: Cow::Borrowed("Details"),
            panel: Some(Cow::Borrowed("<p>Details panel content.</p>")),
        },
    ];

    tabs(
        TabsProps::builder()
            .id(Cow::Borrowed("demo-tabs"))
            .tabsets(tabsets)
            .build(),
    )
}

fn main() {
    let card_html = build_card();
    let tabs_html = build_tabs();

    let page = rsx! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"basecoat-rs static site demo"</title>
                <link
                    rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/basecoat-css@latest/dist/basecoat.css"
                />
                <script src="https://cdn.tailwindcss.com" />
            </head>
            <body class="p-8 space-y-6">
                <h1 class="text-2xl font-bold">"basecoat-rs static demo"</h1>

                <div class="flex gap-3 flex-wrap">
                    <Button variant=ButtonVariant::Primary>"Primary"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                    <Button variant=ButtonVariant::Outline>"Outline"</Button>
                    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                    <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                </div>

                <div class="flex gap-2 flex-wrap">
                    <Badge variant=BadgeVariant::Default>"Default"</Badge>
                    <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                    <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
                    <Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
                </div>

                <div>{card_html}</div>

                <div>{tabs_html}</div>

                <p class="text-sm text-gray-500">
                    "Interactive components (Dialog, Tabs, Toast) hydrate via "
                    <code>"basecoat-controllers.js"</code>
                    " — not loaded in this static demo."
                </p>
            </body>
        </html>
    };

    let html = format!("<!DOCTYPE html>\n{page}");

    let dist = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("dist");
    std::fs::create_dir_all(&dist).expect("failed to create dist/");
    let out = dist.join("index.html");
    std::fs::write(&out, &html).expect("failed to write index.html");
    println!("Written: {}", out.display());
    println!("Size: {} bytes", html.len());
    println!();
    assert!(html.contains("btn-primary"), "expected btn-primary class");
    assert!(
        html.contains("btn-secondary"),
        "expected btn-secondary class"
    );
    assert!(html.contains("btn-outline"), "expected btn-outline class");
    assert!(html.contains("badge"), "expected badge class");
    assert!(html.contains("card"), "expected card class");
    println!("All assertions passed.");
}
