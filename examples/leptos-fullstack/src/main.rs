//! leptos-fullstack — dual entry point.
//!
//! - **SSR** (`--features ssr`): Axum server binary. Build and run via
//!   `cargo leptos serve` or `cargo leptos watch` from this directory.
//! - **Hydrate** (`--features hydrate`, WASM target): exports `hydrate()` for
//!   the browser bundle produced by cargo-leptos.
//! - **Neither**: no-op `main()` so `cargo check --workspace` succeeds without
//!   cargo-leptos installed.
//!
//! # Quick start (after `cargo install cargo-leptos`)
//!
//! ```sh
//! # From the workspace root — build the WASM controllers bundle first:
//! cargo xtask build-wasm
//!
//! # Then launch the full-stack app:
//! cd examples/leptos-fullstack
//! cargo leptos watch
//! # Visit http://127.0.0.1:3001
//! ```

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::{get_configuration, LeptosOptions};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_fullstack::app::{shell, App};
    use leptos_fullstack::fileserv::file_and_error_handler;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Build a typed Router<LeptosOptions> so the LeptosRoutes trait bound
    // (LeptosOptions: FromRef<S>) is satisfied before .leptos_routes is called.
    let app: Router<LeptosOptions> = Router::new();
    let app = app
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
