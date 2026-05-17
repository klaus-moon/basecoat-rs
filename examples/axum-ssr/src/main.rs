//! Axum SSR example — serves an HTML page built with `rsx!` and mounts the
//! WASM controllers bundle so Dialog, Tabs, and Toast hydrate in the browser.
//!
//! # Setup (one-time)
//!
//! 1. Build the CSS bundle from the workspace root:
//! ```sh
//! npm install && npm run build:css
//! ```
//!
//! 2. Build the WASM controllers bundle:
//! ```sh
//! cargo xtask build-wasm
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

use axum::{Router, response::Html, response::Response, routing::get};
use basecoat_rs::components::{dialog, tabs, toaster};
use basecoat_rs::{ButtonVariant, Children, DialogProps, TabSet, TabsProps, ToasterProps, rsx};
use tower_http::services::ServeDir;

/// Compiled CSS embedded at build time.
///
/// Requires `style/dist/basecoat-rs.css` to exist at the workspace root.
/// Run `npm install && npm run build:css` from the workspace root once before
/// building. If the file is missing, `include_bytes!` will fail with a clear
/// path error pointing to the missing file.
const COMPILED_CSS: &[u8] = include_bytes!("../../../style/dist/basecoat-rs.css");

async fn styles_handler() -> Response {
    use axum::http::header;
    let mut resp = Response::new(axum::body::Body::from(COMPILED_CSS));
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        "text/css; charset=utf-8".parse().unwrap(),
    );
    resp
}

#[tokio::main]
async fn main() {
    let static_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/basecoat-controllers/pkg");

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/static/styles.css", get(styles_handler))
        .nest_service("/static", ServeDir::new(static_dir));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind failed");

    println!("Listening on http://localhost:{port}");
    axum::serve(listener, app).await.expect("server error");
}

async fn index_handler() -> Html<String> {
    Html(render_page())
}

fn render_page() -> String {
    // Build interactive components outside rsx! so we can use typed builders
    // with Cow values (string literals in rsx! attributes are &str which does
    // not implement Into<Cow<'static, str>> through Option).
    // `trigger` becomes the OUTSIDE button that opens the dialog;
    // `children` is the dialog body (rendered inside the modal).
    let dialog_html = dialog(
        DialogProps::builder()
            .id(Cow::Borrowed("demo-dialog"))
            .trigger(Some(basecoat_rs::Markup::from("Open Dialog")))
            .title(Cow::Borrowed("Confirm Action"))
            .description(Cow::Borrowed(
                "This dialog is rendered server-side and hydrated by the WASM controller.",
            ))
            .children(Children::from(
                "<p>Press <kbd>Esc</kbd> or click outside the dialog to close.</p>",
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
                <link rel="stylesheet" href="/static/styles.css" />
                <script type="module" src="/static/basecoat-controllers.init.js" />
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
