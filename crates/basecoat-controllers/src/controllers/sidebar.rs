// Sidebar controller
//
// Wires a responsive <aside class="sidebar"> with a sibling toggle button:
//   - Above `breakpoint` (default 768px): in-flow column, no dismiss listener.
//   - Below `breakpoint`: overlay drawer, dismiss-on-outside-click attached.
//   - Toggle button click: flips data-state between "expanded"/"collapsed",
//     updates aria-expanded, persists to localStorage.
//   - localStorage key `basecoat:sidebar:{id}` stores "expanded"|"collapsed";
//     read once on hydrate to restore previous state.
//   - Dispatches CustomEvent("basecoat:initialized") with detail.name="sidebar".
//
// Closure lifetime: page-lifetime listeners (toggle click, media-query
// change) use `Closure::forget()` — same rationale as dialog.rs. The dynamic
// dismiss listener is owned by a `DismissHandle` stored in a `RefCell` so it
// can be detached/re-attached when crossing the breakpoint.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, MouseEvent, Storage};

use super::dismiss::{self, DismissHandle};
use super::media;
use super::util::dispatch_initialized;

const STORAGE_PREFIX: &str = "basecoat:sidebar:";

pub fn attach(root: Element) {
    let id = root.id();
    if id.is_empty() {
        web_sys::console::warn_1(
            &"[basecoat:sidebar] <aside data-basecoat-hydrate=\"sidebar\"> requires an id".into(),
        );
        return;
    }

    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    let aside = match root.clone().dyn_into::<HtmlElement>() {
        Ok(el) => el,
        Err(_) => return,
    };

    // Locate the sibling toggle button (rendered next to the aside).
    let selector = format!(
        "[data-sidebar-toggle=\"{id}\"], button[aria-controls=\"{id}\"]",
        id = id
    );
    let toggle_node = match document.query_selector(&selector) {
        Ok(Some(el)) => el,
        _ => {
            web_sys::console::warn_1(
                &format!("[basecoat:sidebar] no toggle button found for #{id}").into(),
            );
            return;
        }
    };
    let toggle = match toggle_node.dyn_into::<HtmlElement>() {
        Ok(el) => el,
        Err(_) => return,
    };

    // Read breakpoint override from data-sidebar-breakpoint (defaults to 768).
    let breakpoint_px = aside
        .get_attribute("data-sidebar-breakpoint")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(768.0);

    // Determine initial state: localStorage wins, else fall back to the
    // server-rendered data-state attribute.
    let storage_key = format!("{STORAGE_PREFIX}{id}");
    let initial_state = read_state(&storage_key).unwrap_or_else(|| {
        match aside.get_attribute("data-state").as_deref() {
            Some("collapsed") => "collapsed".to_string(),
            _ => "expanded".to_string(),
        }
    });
    apply_state(&aside, &toggle, &initial_state);

    // Shared handle for the dynamic dismiss listener (overlay mode only).
    let dismiss_handle: Rc<RefCell<Option<DismissHandle>>> = Rc::new(RefCell::new(None));

    // ---- Toggle click ------------------------------------------------------
    {
        let aside_c = aside.clone();
        let toggle_c = toggle.clone();
        let storage_key_c = storage_key.clone();
        let on_click = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
            let next = match aside_c.get_attribute("data-state").as_deref() {
                Some("expanded") => "collapsed",
                _ => "expanded",
            };
            apply_state(&aside_c, &toggle_c, next);
            write_state(&storage_key_c, next);
        });
        toggle
            .add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_click.forget();
    }

    // ---- Media-query reconciliation ---------------------------------------
    // Above breakpoint: in-flow; tear down any dismiss listener.
    // Below breakpoint: overlay; attach dismiss for click-outside-to-close.
    let reconcile = {
        let aside_c = aside.clone();
        let toggle_c = toggle.clone();
        let storage_key_c = storage_key.clone();
        let dismiss_c = dismiss_handle.clone();
        move |is_desktop: bool| {
            if is_desktop {
                // Drop any existing overlay listener — the DismissHandle's
                // Drop impl removes the document-level handlers.
                dismiss_c.borrow_mut().take();
            } else {
                // Re-attach if not already attached.
                let mut slot = dismiss_c.borrow_mut();
                if slot.is_some() {
                    return;
                }
                let aside_for_dismiss = aside_c.clone();
                let toggle_for_dismiss = toggle_c.clone();
                let key_for_dismiss = storage_key_c.clone();
                let handle = dismiss::attach(
                    aside_c.clone(),
                    toggle_c.clone(),
                    Box::new(move || {
                        // Only collapse if currently expanded.
                        if aside_for_dismiss.get_attribute("data-state").as_deref()
                            == Some("expanded")
                        {
                            apply_state(&aside_for_dismiss, &toggle_for_dismiss, "collapsed");
                            write_state(&key_for_dismiss, "collapsed");
                        }
                    }),
                );
                *slot = Some(handle);
            }
        }
    };

    let query = format!("(min-width: {breakpoint_px}px)");
    let is_desktop = media::matches(&query);
    reconcile(is_desktop);

    // Subscribe to viewport changes. The handle is intentionally leaked —
    // the listener should live as long as the sidebar element does (page
    // lifetime). The dismiss handle inside `dismiss_handle` is dropped
    // explicitly by `reconcile` when crossing the breakpoint.
    let media_handle = media::on_change(&query, Box::new(reconcile));
    std::mem::forget(media_handle);

    dispatch_initialized(&aside.clone().into(), "sidebar");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn apply_state(aside: &HtmlElement, toggle: &HtmlElement, state: &str) {
    let aside_el: &Element = aside.as_ref();
    let _ = aside_el.set_attribute("data-state", state);
    let toggle_el: &Element = toggle.as_ref();
    let expanded = if state == "expanded" { "true" } else { "false" };
    let _ = toggle_el.set_attribute("aria-expanded", expanded);
}

fn local_storage() -> Option<Storage> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
}

fn read_state(key: &str) -> Option<String> {
    let storage = local_storage()?;
    let value = storage.get_item(key).ok().flatten()?;
    match value.as_str() {
        "expanded" | "collapsed" => Some(value),
        _ => None,
    }
}

fn write_state(key: &str, state: &str) {
    let Some(storage) = local_storage() else {
        return;
    };
    let _ = storage.set_item(key, state);
}
