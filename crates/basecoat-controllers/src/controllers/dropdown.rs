// Dropdown controller
//
// Wires a `<details class="dropdown-menu">` element into a fully behaviored
// menu by composing four Phase 1 primitives:
//
//   * `floating::position` — anchors the `<div role="menu">` to its
//     `<summary>` trigger using `@floating-ui/dom`.
//   * `keyboard::attach` — installs roving-tabindex navigation and
//     type-ahead over the menuitems.
//   * `dismiss::attach` — closes the menu on outside click, Escape, or
//     focus moving outside the menu/trigger pair.
//   * `util::dispatch_initialized` — fires the canonical
//     `basecoat:initialized` CustomEvent once wiring completes.
//
// Lifetime / leak strategy
// ------------------------
// The toggle and item-activation listeners live for the lifetime of the page
// (the `<details>` element itself is page-lived), so we use
// `Closure::forget()` for them (same rationale as `dialog.rs`). The roving
// and dismiss handles are owned per-open: opening the menu installs them,
// closing the menu drops them. We keep them inside a `RefCell<HashMap>` in a
// `thread_local!` keyed by the element id so multiple dropdowns on the same
// page do not clobber each other's state.
//
// We deliberately avoid `web_sys::HtmlDetailsElement` here because the
// `HtmlDetailsElement` feature is not enabled in `basecoat-controllers`'
// `web-sys` features. Instead we treat the `<details>` element as a plain
// `HtmlElement` and read/write the `open` attribute directly — semantically
// equivalent for the HTML spec.

use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlElement};

use super::dismiss::{DismissHandle, attach as dismiss_attach};
use super::floating::position;
use super::keyboard::{Orientation, RovingHandle, RovingOpts, attach as keyboard_attach};
use super::util::dispatch_initialized;

/// Per-instance runtime state. A dropdown owns at most one open menu at a
/// time, so we track the roving + dismiss handles together: dropping the
/// `OpenState` tears both down deterministically.
struct OpenState {
    _roving: RovingHandle,
    _dismiss: DismissHandle,
}

thread_local! {
    static OPEN_STATES: RefCell<HashMap<String, OpenState>> = RefCell::new(HashMap::new());
}

pub fn attach(root: Element) {
    let details = match root.clone().dyn_into::<HtmlElement>() {
        Ok(el) => el,
        Err(_) => {
            web_sys::console::warn_1(
                &"[basecoat:dropdown] root must be an HtmlElement".into(),
            );
            return;
        }
    };
    let id = details.id();
    if id.is_empty() {
        web_sys::console::warn_1(
            &"[basecoat:dropdown] <details data-basecoat-hydrate=\"dropdown\"> requires an id"
                .into(),
        );
        return;
    }

    let Some(summary) = query_summary(&details) else {
        web_sys::console::warn_1(
            &format!("[basecoat:dropdown] #{id} is missing <summary>").into(),
        );
        return;
    };
    let Some(menu) = query_menu(&details) else {
        web_sys::console::warn_1(
            &format!("[basecoat:dropdown] #{id} is missing [role=\"menu\"]").into(),
        );
        return;
    };

    let placement = details
        .get_attribute("data-placement")
        .unwrap_or_else(|| "bottom-start".to_string());

    // Initial ARIA / state mirroring.
    let _ = summary.set_attribute("aria-expanded", "false");
    let _ = details.set_attribute("data-state", "closed");

    // ---- toggle listener on <details> --------------------------------------
    // `<details>` fires a `toggle` event whenever its `open` attribute
    // changes (either via user click on <summary> or programmatic mutation).
    let details_for_toggle = details.clone();
    let summary_for_toggle = summary.clone();
    let menu_for_toggle = menu.clone();
    let id_for_toggle = id.clone();
    let placement_for_toggle = placement.clone();
    let on_toggle = Closure::<dyn Fn()>::new(move || {
        if details_is_open(&details_for_toggle) {
            open_menu(
                &id_for_toggle,
                &details_for_toggle,
                &summary_for_toggle,
                &menu_for_toggle,
                &placement_for_toggle,
            );
        } else {
            close_menu(&id_for_toggle, &details_for_toggle, &summary_for_toggle);
        }
    });
    let toggle_target: &web_sys::EventTarget = details.as_ref();
    let _ = toggle_target
        .add_event_listener_with_callback("toggle", on_toggle.as_ref().unchecked_ref());
    on_toggle.forget();

    // ---- item activation closes the menu ----------------------------------
    let details_for_activate = details.clone();
    let summary_for_activate = summary.clone();
    let id_for_activate = id.clone();
    let on_menu_click =
        Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target() else {
                return;
            };
            let Ok(node) = target.dyn_into::<web_sys::Node>() else {
                return;
            };
            let Some(element) = node.dyn_ref::<Element>() else {
                return;
            };
            if element.get_attribute("role").as_deref() != Some("menuitem") {
                return;
            }
            if element.get_attribute("aria-disabled").as_deref() == Some("true") {
                return;
            }
            request_close(
                &id_for_activate,
                &details_for_activate,
                &summary_for_activate,
            );
        });
    let menu_target: &web_sys::EventTarget = menu.as_ref();
    let _ = menu_target
        .add_event_listener_with_callback("click", on_menu_click.as_ref().unchecked_ref());
    on_menu_click.forget();

    dispatch_initialized(&details.clone().into(), "dropdown");
}

// ---------------------------------------------------------------------------
// Open / close helpers
// ---------------------------------------------------------------------------

fn open_menu(
    id: &str,
    details: &HtmlElement,
    summary: &HtmlElement,
    menu: &HtmlElement,
    placement: &str,
) {
    let _ = summary.set_attribute("aria-expanded", "true");
    let _ = details.set_attribute("data-state", "open");

    // ---- floating positioning ---------------------------------------------
    let summary_ref: Element = summary.clone().into();
    let menu_for_position = menu.clone();
    let placement_owned = placement.to_string();
    spawn_local(async move {
        position(&summary_ref, &menu_for_position, &placement_owned).await;
    });

    // ---- roving tabindex + type-ahead -------------------------------------
    let items = collect_items(menu);
    let roving = keyboard_attach(
        menu.as_ref(),
        items,
        RovingOpts {
            orientation: Orientation::Vertical,
            wrap: true,
            type_ahead: true,
        },
    );

    // Focus the first enabled menuitem.
    focus_first_menuitem(menu);

    // ---- dismiss listeners ------------------------------------------------
    let id_for_dismiss = id.to_string();
    let details_for_dismiss = details.clone();
    let summary_for_dismiss = summary.clone();
    let dismiss = dismiss_attach(
        menu.clone(),
        summary.clone(),
        Box::new(move || {
            request_close(&id_for_dismiss, &details_for_dismiss, &summary_for_dismiss);
        }),
    );

    OPEN_STATES.with(|states| {
        states.borrow_mut().insert(
            id.to_string(),
            OpenState {
                _roving: roving,
                _dismiss: dismiss,
            },
        );
    });
}

fn close_menu(id: &str, details: &HtmlElement, summary: &HtmlElement) {
    let _ = summary.set_attribute("aria-expanded", "false");
    let _ = details.set_attribute("data-state", "closed");
    OPEN_STATES.with(|states| {
        states.borrow_mut().remove(id);
    });
}

/// Programmatic close: remove `<details open>`, which will fire `toggle` and
/// run `close_menu` for ARIA + state cleanup. We also focus the summary
/// immediately so keyboard users land on a sensible element.
fn request_close(id: &str, details: &HtmlElement, summary: &HtmlElement) {
    if !details_is_open(details) {
        // Already closed — still flush any leftover open-state listeners.
        close_menu(id, details, summary);
        return;
    }
    let _ = details.remove_attribute("open");
    let _ = summary.focus();
}

fn details_is_open(details: &HtmlElement) -> bool {
    details.has_attribute("open")
}

// ---------------------------------------------------------------------------
// DOM query helpers
// ---------------------------------------------------------------------------

fn query_summary(root: &HtmlElement) -> Option<HtmlElement> {
    let root_el: &Element = root.as_ref();
    root_el
        .query_selector("summary")
        .ok()
        .flatten()
        .and_then(|n| n.dyn_into::<HtmlElement>().ok())
}

fn query_menu(root: &HtmlElement) -> Option<HtmlElement> {
    let root_el: &Element = root.as_ref();
    root_el
        .query_selector("[role=\"menu\"]")
        .ok()
        .flatten()
        .and_then(|n| n.dyn_into::<HtmlElement>().ok())
}

fn collect_items(menu: &HtmlElement) -> Vec<HtmlElement> {
    let menu_el: &Element = menu.as_ref();
    let Ok(node_list) = menu_el.query_selector_all("[role=\"menuitem\"]") else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(node_list.length() as usize);
    for i in 0..node_list.length() {
        if let Some(node) = node_list.item(i)
            && let Ok(el) = node.dyn_into::<HtmlElement>()
        {
            out.push(el);
        }
    }
    out
}

fn focus_first_menuitem(menu: &HtmlElement) {
    let items = collect_items(menu);
    let target = items.iter().find(|el| is_enabled(el));
    if let Some(el) = target {
        let _ = el.focus();
    }
}

fn is_enabled(el: &HtmlElement) -> bool {
    if el.has_attribute("disabled") {
        return false;
    }
    if el.get_attribute("aria-disabled").as_deref() == Some("true") {
        return false;
    }
    true
}
