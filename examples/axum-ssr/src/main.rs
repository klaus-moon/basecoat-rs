//! Axum SSR example — serves an HTML page built with `rsx!` and mounts the
//! WASM controllers bundle so Dialog, Tabs, and Toast hydrate in the browser.
//!
//! # Setup (one-time)
//!
//! Build the WASM bundle before running this server:
//! ```sh
//! wasm-pack build --release --target web crates/basecoat-controllers
//! ```
//!
//! # Run
//!
//! ```sh
//! cargo run -p axum-ssr
//! # then open http://localhost:3000
//! ```

// rsx! expression blocks `{expr}` trigger unused_braces for single identifiers.
#![allow(unused_braces)]

use std::borrow::Cow;

use axum::{Router, response::Html, routing::get};
use basecoat_rs::components::{dialog, tabs, toaster};
use basecoat_rs::{ButtonVariant, Children, DialogProps, TabSet, TabsProps, ToasterProps, rsx};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let static_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/basecoat-controllers/pkg");

    let app = Router::new()
        .route("/", get(index_handler))
        .nest_service("/static", ServeDir::new(static_dir));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind port 3000");

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.expect("server error");
}

async fn index_handler() -> Html<String> {
    Html(render_page())
}

fn render_page() -> String {
    // Build interactive components outside rsx! so we can use typed builders
    // with Cow values (string literals in rsx! attributes are &str which does
    // not implement Into<Cow<'static, str>> through Option).
    let dialog_html = dialog(
        DialogProps::builder()
            .id(Cow::Borrowed("demo-dialog"))
            .title(Cow::Borrowed("Confirm Action"))
            .description(Cow::Borrowed(
                "This dialog is rendered server-side and hydrated by the WASM controller.",
            ))
            .children(Children::from(
                r#"<button class="btn-primary" data-dialog-trigger="demo-dialog">Open Dialog</button>"#.to_owned(),
            ))
            .build(),
    );

    let tabsets = vec![
        TabSet {
            tab: Cow::Borrowed("Account"),
            panel: Some(Cow::Borrowed(
                "<p class=\"p-4\">Manage your account settings here.</p>",
            )),
        },
        TabSet {
            tab: Cow::Borrowed("Security"),
            panel: Some(Cow::Borrowed(
                "<p class=\"p-4\">Update your password and two-factor settings.</p>",
            )),
        },
        TabSet {
            tab: Cow::Borrowed("Notifications"),
            panel: Some(Cow::Borrowed(
                "<p class=\"p-4\">Configure your notification preferences.</p>",
            )),
        },
    ];

    let tabs_html = tabs(
        TabsProps::builder()
            .id(Cow::Borrowed("settings-tabs"))
            .tabsets(tabsets)
            .build(),
    );

    let toaster_html = toaster(
        ToasterProps::builder()
            .id(Cow::Borrowed("global-toaster"))
            .build(),
    );

    let markup = rsx! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"basecoat-rs Axum SSR demo"</title>
                <link
                    rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/basecoat-css@latest/dist/basecoat.css"
                />
                <script src="https://cdn.tailwindcss.com" />
                <script type="module" src="/static/basecoat_controllers.js" />
            </head>
            <body class="p-8 max-w-2xl mx-auto space-y-8">
                <h1 class="text-3xl font-bold">"basecoat-rs · Axum SSR"</h1>

                <section class="space-y-3">
                    <h2 class="text-xl font-semibold">"Buttons"</h2>
                    <div class="flex gap-3 flex-wrap">
                        <Button variant=ButtonVariant::Primary>"Save"</Button>
                        <Button variant=ButtonVariant::Secondary>"Cancel"</Button>
                        <Button variant=ButtonVariant::Outline>"Preview"</Button>
                        <Button variant=ButtonVariant::Ghost>"Dismiss"</Button>
                        <Button variant=ButtonVariant::Destructive>"Delete"</Button>
                    </div>
                </section>

                <section class="space-y-3">
                    <h2 class="text-xl font-semibold">"Tabs (hydrated)"</h2>
                    <div>{tabs_html}</div>
                </section>

                <section class="space-y-3">
                    <h2 class="text-xl font-semibold">"Dialog (hydrated)"</h2>
                    <div>{dialog_html}</div>
                </section>

                <section class="space-y-3">
                    <h2 class="text-xl font-semibold">"Toast (JS API)"</h2>
                    <button
                        class="btn-outline"
                        onclick="window.basecoat.toast({ title: 'Hello!', description: 'Toast triggered from JS.', category: 'success' })"
                    >
                        "Show Toast"
                    </button>
                    <div>{toaster_html}</div>
                </section>
            </body>
        </html>
    };

    format!("<!DOCTYPE html>\n{markup}")
}
