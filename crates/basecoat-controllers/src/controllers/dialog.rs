// Dialog controller
//
// Wires a native <dialog> element with:
//   - Trigger click → showModal()
//   - Escape / backdrop click → close()
//   - Focus trap (Tab/Shift+Tab cycle among focusable descendants)
//   - data-state="open"|"closed" mirroring
//   - CustomEvent("basecoat:initialized") after wiring
//
// Closure lifetime strategy: `Closure::forget()`
//   Rationale: event listeners on DOM elements must outlive the Rust stack.
//   We intentionally leak closures here (the DOM element itself lives as long
//   as the page, so there is no additional unbounded growth). A
//   `thread_local! { static CLOSURES: RefCell<Vec<...>> }` alternative is
//   structurally equivalent but adds indirection with no practical benefit for
//   page-lifetime listeners. We document the leak explicitly so future
//   maintainers can add teardown if dynamic detachment becomes a requirement.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlDialogElement, HtmlElement, KeyboardEvent, MouseEvent};

use super::util::dispatch_initialized;

const FOCUSABLE: &str = concat!(
    "a[href],button:not([disabled]),input:not([disabled]),",
    "select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex='-1'])"
);

pub fn attach(root: Element) {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    // Two supported root shapes:
    //   (a) root is a wrapper containing both the [data-dialog-trigger]
    //       button and the <dialog> element (basecoat-components convention).
    //   (b) root IS the <dialog> element, and the trigger is a sibling found
    //       globally via [data-dialog-trigger="{root.id}"] OR
    //       [data-dialog-trigger][aria-controls="{root.id}"]
    //       (basecoat-leptos DialogTrigger + DialogContent convention).
    let root_is_dialog = root.dyn_ref::<HtmlDialogElement>().is_some();

    let (trigger, dialog): (HtmlElement, HtmlDialogElement) = if root_is_dialog {
        let dialog_id = root.id();
        if dialog_id.is_empty() {
            web_sys::console::warn_1(
                &"[basecoat:dialog] <dialog data-basecoat-hydrate> requires an id".into(),
            );
            return;
        }
        let selector =
            format!("[data-dialog-trigger=\"{dialog_id}\"], [aria-controls=\"{dialog_id}\"]");
        let trigger_node = match document.query_selector(&selector) {
            Ok(Some(el)) => el,
            _ => {
                web_sys::console::warn_1(
                    &format!("[basecoat:dialog] no trigger found for #{dialog_id}").into(),
                );
                return;
            }
        };
        let trigger = match trigger_node.dyn_into::<HtmlElement>() {
            Ok(e) => e,
            Err(_) => return,
        };
        let dialog = match root.clone().dyn_into::<HtmlDialogElement>() {
            Ok(d) => d,
            Err(_) => return,
        };
        (trigger, dialog)
    } else {
        let trigger_node = match root.query_selector("[data-dialog-trigger]") {
            Ok(Some(el)) => el,
            _ => {
                web_sys::console::warn_1(
                    &"[basecoat:dialog] no [data-dialog-trigger] found".into(),
                );
                return;
            }
        };
        let trigger = match trigger_node.dyn_into::<HtmlElement>() {
            Ok(e) => e,
            Err(_) => return,
        };
        let dialog_id = match trigger.get_attribute("aria-controls") {
            Some(id) => id,
            None => {
                web_sys::console::warn_1(
                    &"[basecoat:dialog] trigger missing aria-controls".into(),
                );
                return;
            }
        };
        let dialog_el = match document.get_element_by_id(&dialog_id) {
            Some(el) => el,
            None => {
                web_sys::console::warn_1(
                    &format!("[basecoat:dialog] no element with id=\"{dialog_id}\"").into(),
                );
                return;
            }
        };
        let dialog = match dialog_el.dyn_into::<HtmlDialogElement>() {
            Ok(d) => d,
            Err(_) => {
                web_sys::console::warn_1(
                    &format!("[basecoat:dialog] #{dialog_id} is not a <dialog>").into(),
                );
                return;
            }
        };
        (trigger, dialog)
    };

    set_state(&dialog.clone().into(), "closed");

    // ---- Trigger click: open dialog ----------------------------------------
    {
        let dialog_c = dialog.clone();
        let trigger_c = trigger.clone();
        let on_trigger_click = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
            let _ = dialog_c.show_modal();
            set_state(&dialog_c.clone().into(), "open");
            focus_first(&dialog_c.clone().into());
            let _ = trigger_c.blur();
        });
        trigger
            .add_event_listener_with_callback("click", on_trigger_click.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_trigger_click.forget();
    }

    // ---- Backdrop click: close dialog --------------------------------------
    {
        let dialog_c = dialog.clone();
        let trigger_c = trigger.clone();
        let on_backdrop_click = Closure::<dyn Fn(MouseEvent)>::new(move |e: MouseEvent| {
            let dialog_as_el: &Element = dialog_c.as_ref();
            let rect = dialog_as_el.get_bounding_client_rect();
            // Native dialog backdrop click: the click target is the dialog
            // element itself but the coords fall outside its bounding rect.
            let x = e.client_x() as f64;
            let y = e.client_y() as f64;
            let outside =
                x < rect.left() || x > rect.right() || y < rect.top() || y > rect.bottom();
            if outside {
                dialog_c.close();
                set_state(&dialog_c.clone().into(), "closed");
                let _ = trigger_c.focus();
            }
        });
        let dialog_el: &web_sys::EventTarget = dialog.as_ref();
        dialog_el
            .add_event_listener_with_callback("click", on_backdrop_click.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_backdrop_click.forget();
    }

    // ---- Keydown: Escape + focus-trap ---------------------------------------
    {
        let dialog_c = dialog.clone();
        let trigger_c = trigger.clone();
        let on_keydown = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Escape" => {
                    // Browser closes the dialog natively on Escape, but we
                    // still need to sync data-state and return focus.
                    set_state(&dialog_c.clone().into(), "closed");
                    let _ = trigger_c.focus();
                }
                "Tab" => {
                    e.prevent_default();
                    trap_focus(&dialog_c.clone().into(), e.shift_key());
                }
                _ => {}
            }
        });
        let dialog_el: &web_sys::EventTarget = dialog.as_ref();
        dialog_el
            .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_keydown.forget();
    }

    // ---- Also sync data-state on native "close" event ----------------------
    {
        let dialog_c = dialog.clone();
        let trigger_c = trigger.clone();
        let on_close = Closure::<dyn Fn()>::new(move || {
            set_state(&dialog_c.clone().into(), "closed");
            let _ = trigger_c.focus();
        });
        let dialog_el: &web_sys::EventTarget = dialog.as_ref();
        dialog_el
            .add_event_listener_with_callback("close", on_close.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_close.forget();
    }

    // ---- Dispatch initialized event ----------------------------------------
    dispatch_initialized(&dialog.into(), "dialog");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn set_state(el: &Element, state: &str) {
    let _ = el.set_attribute("data-state", state);
}

fn focusable_elements(container: &Element) -> Vec<HtmlElement> {
    let node_list = match container.query_selector_all(FOCUSABLE) {
        Ok(node_list) => node_list,
        Err(_) => return vec![],
    };
    let mut out = Vec::new();
    for i in 0..node_list.length() {
        if let Some(n) = node_list.item(i)
            && let Ok(el) = n.dyn_into::<HtmlElement>()
        {
            out.push(el);
        }
    }
    out
}

fn focus_first(container: &Element) {
    let focusables = focusable_elements(container);
    if let Some(first) = focusables.first() {
        let _ = first.focus();
    }
}

fn trap_focus(container: &Element, shift: bool) {
    let focusables = focusable_elements(container);
    if focusables.is_empty() {
        return;
    }
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };
    let active = document.active_element();
    let current_idx = active.as_ref().and_then(|a| {
        focusables.iter().position(|e| {
            let e_el: &Element = e.as_ref();
            e_el == a
        })
    });

    let next = match current_idx {
        Some(idx) => {
            if shift {
                if idx == 0 {
                    focusables.len() - 1
                } else {
                    idx - 1
                }
            } else if idx == focusables.len() - 1 {
                0
            } else {
                idx + 1
            }
        }
        None => 0,
    };
    let _ = focusables[next].focus();
}

