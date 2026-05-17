//! Static file fallback handler for the SSR Axum server.
//!
//! Serves files from the cargo-leptos `site-root` (`target/site`) for any URL
//! that is not a Leptos route. Also serves `/static/*` from the
//! `crates/basecoat-controllers/pkg/` directory so the WASM controllers JS
//! bundle is reachable at `/static/basecoat_controllers.js`.

use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use leptos::config::LeptosOptions;
use tower::ServiceExt;
use tower_http::services::ServeDir;

/// Fallback handler: tries to serve a static file from `site-root`; if the
/// file is not found, serves a plain 404 response.
pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    _req: Request<Body>,
) -> Response<Body> {
    let root = options.site_root.to_string();

    // Attempt to serve the WASM controllers bundle at /static/*
    let static_prefix = "/static/";
    if let Some(path) = uri.path().strip_prefix(static_prefix) {
        let controllers_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/basecoat-controllers/pkg");
        let static_uri: Uri = format!("/{path}").parse().unwrap_or_else(|_| uri.clone());
        if let Ok(resp) =
            get_static_file(static_uri, controllers_dir.to_str().unwrap_or(".")).await
        {
            if resp.status() == StatusCode::OK {
                return resp;
            }
        }
    }

    // Fall back to the cargo-leptos site-root.
    if let Ok(resp) = get_static_file(uri, &root).await {
        if resp.status() == StatusCode::OK {
            return resp;
        }
    }

    (StatusCode::NOT_FOUND, "Not found").into_response()
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .expect("request builder");
    ServeDir::new(root)
        .oneshot(req)
        .await
        .map(|resp| resp.map(Body::new))
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
