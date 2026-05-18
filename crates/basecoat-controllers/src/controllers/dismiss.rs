// Dismiss helper — click-outside, Escape, and focus-out.
//
// Centralizes the "close this floating surface when the user looks
// elsewhere" logic shared by Popover, Dropdown, Select, and Combobox.
//
// Closure lifetime: `DismissHandle` owns every `Closure` and removes the
// listeners on `Drop`. This is INTENTIONALLY DIFFERENT from dialog.rs's
// `Closure::forget()` pattern: dialogs live for the lifetime of the page,
// but popovers are opened and closed dynamically — leaking a click-outside
// listener for every popover would grow unbounded. Always store the closure
// inside the handle; never call `Closure::forget()` here.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Document, FocusEvent, HtmlElement, KeyboardEvent, MouseEvent, Node};

pub struct DismissHandle {
    document: Document,
    click: Option<Closure<dyn Fn(MouseEvent)>>,
    keydown: Option<Closure<dyn Fn(KeyboardEvent)>>,
    focusin: Option<Closure<dyn Fn(FocusEvent)>>,
}

impl Drop for DismissHandle {
    fn drop(&mut self) {
        let target: &web_sys::EventTarget = self.document.as_ref();
        if let Some(c) = self.click.take() {
            // Click listener was registered in capture phase — must remove
            // with the same flag, otherwise removeEventListener is a no-op.
            let _ = target.remove_event_listener_with_callback_and_bool(
                "click",
                c.as_ref().unchecked_ref(),
                true,
            );
        }
        if let Some(c) = self.keydown.take() {
            let _ = target
                .remove_event_listener_with_callback("keydown", c.as_ref().unchecked_ref());
        }
        if let Some(c) = self.focusin.take() {
            let _ = target
                .remove_event_listener_with_callback("focusin", c.as_ref().unchecked_ref());
        }
    }
}

/// Wire dismiss listeners on `document`. `on_dismiss` is invoked when:
/// - The user clicks outside both `floating` and `trigger` (capture phase).
/// - The user presses Escape while `floating` contains focus or is open.
/// - Focus leaves both `floating` and `trigger` (focusin elsewhere).
pub fn attach(
    floating: HtmlElement,
    trigger: HtmlElement,
    on_dismiss: Box<dyn Fn()>,
) -> DismissHandle {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .expect("dismiss::attach requires a document");

    // Wrap the user callback so all three listeners can share it.
    let on_dismiss = std::rc::Rc::<dyn Fn()>::from(on_dismiss);

    let click = wire_click(&document, floating.clone(), trigger.clone(), on_dismiss.clone());
    let keydown = wire_keydown(&document, floating.clone(), on_dismiss.clone());
    let focusin = wire_focusin(&document, floating, trigger, on_dismiss);

    DismissHandle {
        document,
        click: Some(click),
        keydown: Some(keydown),
        focusin: Some(focusin),
    }
}

fn wire_click(
    document: &Document,
    floating: HtmlElement,
    trigger: HtmlElement,
    on_dismiss: std::rc::Rc<dyn Fn()>,
) -> Closure<dyn Fn(MouseEvent)> {
    let closure = Closure::<dyn Fn(MouseEvent)>::new(move |e: MouseEvent| {
        let Some(target_value) = e.target() else {
            return;
        };
        let Ok(node) = target_value.dyn_into::<Node>() else {
            return;
        };
        if contains(&floating, &node) || contains(&trigger, &node) {
            return;
        }
        on_dismiss();
    });
    let target: &web_sys::EventTarget = document.as_ref();
    let _ = target.add_event_listener_with_callback_and_bool(
        "click",
        closure.as_ref().unchecked_ref(),
        true, // capture phase
    );
    closure
}

fn wire_keydown(
    document: &Document,
    floating: HtmlElement,
    on_dismiss: std::rc::Rc<dyn Fn()>,
) -> Closure<dyn Fn(KeyboardEvent)> {
    let closure = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        if e.key() != "Escape" {
            return;
        }
        if !floating_is_active(&floating) {
            return;
        }
        e.prevent_default();
        on_dismiss();
    });
    let target: &web_sys::EventTarget = document.as_ref();
    let _ =
        target.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
    closure
}

fn wire_focusin(
    document: &Document,
    floating: HtmlElement,
    trigger: HtmlElement,
    on_dismiss: std::rc::Rc<dyn Fn()>,
) -> Closure<dyn Fn(FocusEvent)> {
    let closure = Closure::<dyn Fn(FocusEvent)>::new(move |e: FocusEvent| {
        let Some(target_value) = e.target() else {
            return;
        };
        let Ok(node) = target_value.dyn_into::<Node>() else {
            return;
        };
        if contains(&floating, &node) || contains(&trigger, &node) {
            return;
        }
        on_dismiss();
    });
    let target: &web_sys::EventTarget = document.as_ref();
    let _ =
        target.add_event_listener_with_callback("focusin", closure.as_ref().unchecked_ref());
    closure
}

fn floating_is_active(floating: &HtmlElement) -> bool {
    // "Open" if data-state="open" or contains the currently focused element.
    if floating.get_attribute("data-state").as_deref() == Some("open") {
        return true;
    }
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return false;
    };
    let Some(active) = document.active_element() else {
        return false;
    };
    let active_node: &Node = active.as_ref();
    contains(floating, active_node)
}

fn contains(container: &HtmlElement, node: &Node) -> bool {
    let container_node: &Node = container.as_ref();
    container_node.contains(Some(node))
}
