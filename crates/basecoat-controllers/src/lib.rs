// basecoat-controllers — WASM entry point.
//
// On start, scans the document for [data-basecoat-hydrate] elements and
// dispatches to the matching controller's attach() function.
//
// Re-hydration: window.basecoat.hydrate() can be called after AJAX content
// swaps to wire newly-inserted elements without re-processing already-wired
// ones (idempotency guard via data-basecoat-hydrated="true").

#[cfg(feature = "panic-hook")]
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use web_sys::{Document, NodeList};

pub mod controllers {
    #[cfg(feature = "dialog")]
    pub mod dialog;
    #[cfg(feature = "tabs")]
    pub mod tabs;
    #[cfg(feature = "toast")]
    pub mod toast;
}

// Re-export for wasm-bindgen-test and downstream users.
#[cfg(feature = "dialog")]
pub use controllers::dialog;
#[cfg(feature = "tabs")]
pub use controllers::tabs;
#[cfg(feature = "toast")]
pub use controllers::toast;

// ---------------------------------------------------------------------------
// WASM start hook
// ---------------------------------------------------------------------------

#[wasm_bindgen(start)]
pub fn __basecoat_start() {
    #[cfg(feature = "panic-hook")]
    console_error_panic_hook::set_once();

    // If the script loads from <head>, the body isn't parsed yet — defer the
    // initial scan to DOMContentLoaded. Otherwise (script at end of body, or
    // injected late) run immediately.
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    if document.ready_state() == "loading" {
        let listener = Closure::<dyn Fn()>::new(|| {
            hydrate();
        });
        let _ = document.add_event_listener_with_callback(
            "DOMContentLoaded",
            listener.as_ref().unchecked_ref(),
        );
        listener.forget(); // listener must outlive the closure scope
    } else {
        hydrate();
    }
    // Always expose window.basecoat synchronously so consumers can call
    // window.basecoat.toast() etc. before the deferred scan fires.
    expose_window_api(&window);
}

// ---------------------------------------------------------------------------
// Public hydration entry point (also exposed on window.basecoat)
// ---------------------------------------------------------------------------

#[wasm_bindgen]
pub fn hydrate() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document: Document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let nodes: NodeList = match document.query_selector_all("[data-basecoat-hydrate]") {
        Ok(nl) => nl,
        Err(_) => return,
    };

    for i in 0..nodes.length() {
        let node = match nodes.item(i) {
            Some(n) => n,
            None => continue,
        };
        let el = match node.dyn_into::<web_sys::Element>() {
            Ok(e) => e,
            Err(_) => continue,
        };

        // Idempotency: skip already-wired elements.
        if el.get_attribute("data-basecoat-hydrated").as_deref() == Some("true") {
            continue;
        }

        // Version check.
        let version = el.get_attribute("data-basecoat-version");
        if version.as_deref() != Some("0.1") {
            web_sys::console::warn_1(
                &format!(
                    "[basecoat] element {:?} has unexpected version {:?}; expected \"0.1\"",
                    el.get_attribute("id"),
                    version
                )
                .into(),
            );
        }

        let name = match el.get_attribute("data-basecoat-hydrate") {
            Some(n) => n,
            None => continue,
        };

        match name.as_str() {
            #[cfg(feature = "dialog")]
            "dialog" => controllers::dialog::attach(el.clone()),
            #[cfg(feature = "tabs")]
            "tabs" => controllers::tabs::attach(el.clone()),
            #[cfg(feature = "toast")]
            "toast" => controllers::toast::attach(el.clone()),
            other => {
                web_sys::console::warn_1(
                    &format!("[basecoat] unknown controller \"{}\"", other).into(),
                );
                continue;
            }
        }

        // Mark as hydrated so re-calls are idempotent.
        let _ = el.set_attribute("data-basecoat-hydrated", "true");
    }
}

fn expose_window_api(window: &web_sys::Window) {
    use js_sys::Reflect;

    let basecoat_obj = js_sys::Object::new();

    // window.basecoat.hydrate
    let hydrate_fn = Closure::<dyn Fn()>::new(|| {
        hydrate();
    });
    let _ = Reflect::set(
        &basecoat_obj,
        &JsValue::from_str("hydrate"),
        hydrate_fn.as_ref(),
    );
    hydrate_fn.forget(); // must live forever

    // window.basecoat.toast (wired by toast module if enabled)
    #[cfg(feature = "toast")]
    controllers::toast::expose_toast_api(&basecoat_obj);

    let _ = Reflect::set(window, &JsValue::from_str("basecoat"), &basecoat_obj);
}
