// Toast controller
//
// Wires a [data-toaster] host for auto-dismissing toast notifications.
//
// Behaviour:
//   - Toasts appended as [data-toast] children of [data-toaster].
//   - Auto-dismiss after 5000ms (configurable via data-duration="<ms>").
//   - Pause timer on mouseenter; resume on mouseleave.
//   - [data-toast-close] button manually dismisses.
//   - Newest toast on top (prepend). Max 5 visible; older ones queue.
//   - data-state="open"|"closed" toggled for CSS enter/exit transitions.
//   - window.basecoat.toast({ title, description, category, duration, action })
//     programmatic API (action: { label, on_click_event_name }).
//
// Closure lifetime: Closure::forget() — see dialog.rs for rationale.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, Element, MouseEvent};

const MAX_VISIBLE: usize = 5;
const DEFAULT_DURATION_MS: u32 = 5000;

pub fn attach(root: Element) {
    // The root element should be (or contain) the toaster.
    let toaster = if root.get_attribute("data-toaster").is_some() {
        root.clone()
    } else {
        match root.query_selector("[data-toaster]") {
            Ok(Some(el)) => el,
            _ => {
                web_sys::console::warn_1(&"[basecoat:toast] no [data-toaster] found".into());
                return;
            }
        }
    };

    // Wire any pre-rendered toasts (SSR case).
    wire_existing_toasts(&toaster);

    // Dispatch initialized on the host.
    dispatch_initialized(&root, "toast");
}

/// Called by lib.rs to add window.basecoat.toast to the basecoat object.
pub fn expose_toast_api(basecoat_obj: &js_sys::Object) {
    let toast_fn = Closure::<dyn Fn(JsValue)>::new(move |opts: JsValue| {
        fire_toast(opts);
    });
    let _ = js_sys::Reflect::set(basecoat_obj, &JsValue::from_str("toast"), toast_fn.as_ref());
    toast_fn.forget();
}

// ---------------------------------------------------------------------------
// Programmatic toast creation
// ---------------------------------------------------------------------------

fn fire_toast(opts: JsValue) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    // Find toaster in document.
    let toaster = match document.query_selector("[data-toaster]") {
        Ok(Some(el)) => el,
        _ => {
            web_sys::console::warn_1(&"[basecoat:toast] no [data-toaster] in document".into());
            return;
        }
    };

    let get = |key: &str| -> Option<String> {
        js_sys::Reflect::get(&opts, &JsValue::from_str(key))
            .ok()
            .and_then(|v| v.as_string())
    };
    let get_u32 = |key: &str| -> Option<u32> {
        js_sys::Reflect::get(&opts, &JsValue::from_str(key))
            .ok()
            .and_then(|v| v.as_f64())
            .map(|f| f as u32)
    };

    let title = get("title").unwrap_or_default();
    let description = get("description").unwrap_or_default();
    let category = get("category").unwrap_or_default();
    let duration = get_u32("duration").unwrap_or(DEFAULT_DURATION_MS);

    // Build action if present.
    let action_html = js_sys::Reflect::get(&opts, &JsValue::from_str("action"))
        .ok()
        .filter(|v| !v.is_undefined() && !v.is_null())
        .and_then(|action_obj| {
            let label = js_sys::Reflect::get(&action_obj, &JsValue::from_str("label"))
                .ok()
                .and_then(|v| v.as_string())?;
            let event_name =
                js_sys::Reflect::get(&action_obj, &JsValue::from_str("on_click_event_name"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();
            Some((label, event_name))
        });

    // Build toast element.
    let toast_el = match document.create_element("div") {
        Ok(el) => el,
        Err(_) => return,
    };
    let _ = toast_el.set_attribute("class", "toast");
    let _ = toast_el.set_attribute("data-toast", "");
    let _ = toast_el.set_attribute("role", "status");
    let _ = toast_el.set_attribute("aria-live", "polite");
    let _ = toast_el.set_attribute("data-state", "open");
    if !category.is_empty() {
        let _ = toast_el.set_attribute("data-category", &category);
    }
    let _ = toast_el.set_attribute("data-duration", &duration.to_string());

    // Inner HTML: title + optional description + close button + optional action.
    let mut inner = format!(r#"<div data-toast-title>{}</div>"#, html_escape(&title));
    if !description.is_empty() {
        inner.push_str(&format!(
            r#"<div data-toast-description>{}</div>"#,
            html_escape(&description)
        ));
    }
    if let Some((label, _)) = &action_html {
        inner.push_str(&format!(
            r#"<button data-toast-action type="button">{}</button>"#,
            html_escape(label)
        ));
    }
    inner.push_str(r#"<button data-toast-close type="button" aria-label="Close">×</button>"#);
    toast_el.set_inner_html(&inner);

    // Wire action button custom event dispatch.
    if let Some((_, event_name)) = &action_html
        && let Ok(Some(btn)) = toast_el.query_selector("[data-toast-action]")
    {
        let event_name_owned = event_name.clone();
        let toast_ref = toast_el.clone();
        let on_action = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
            let init = CustomEventInit::new();
            init.set_bubbles(true);
            if let Ok(ev) = CustomEvent::new_with_event_init_dict(&event_name_owned, &init) {
                let target: &web_sys::EventTarget = toast_ref.as_ref();
                let _ = target.dispatch_event(&ev);
            }
        });
        let btn_el: &web_sys::EventTarget = btn.as_ref();
        btn_el
            .add_event_listener_with_callback("click", on_action.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_action.forget();
    }

    // Enforce max visible; if at limit, queue by hiding overflow (remove oldest).
    enforce_max(&toaster);

    // Prepend (newest on top).
    let first_child = toaster.first_child();
    if let Some(ref fc) = first_child {
        let _ = toaster.insert_before(toast_el.as_ref(), Some(fc));
    } else {
        let _ = toaster.append_child(toast_el.as_ref());
    }

    wire_toast(&toast_el, duration);
}

// ---------------------------------------------------------------------------
// Wire a single toast (dismiss timer, hover pause, close button)
// ---------------------------------------------------------------------------

fn wire_toast(toast_el: &Element, duration: u32) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    // Use a shared remaining-time mechanism via JS closures + timeout IDs stored
    // as data attributes (simplest approach without SharedArrayBuffer).
    // We store the timeout handle in a Rc<Cell> shared across closures.
    use std::cell::Cell;
    use std::rc::Rc;

    let remaining = Rc::new(Cell::new(duration as f64));
    let start_time = Rc::new(Cell::new(js_sys::Date::now()));
    let timeout_id: Rc<Cell<i32>> = Rc::new(Cell::new(-1));

    // Close function.
    let toast_for_close = toast_el.clone();
    let close_fn = Rc::new(move || {
        dismiss_toast(&toast_for_close);
    });

    // Schedule dismiss.
    let schedule = {
        let remaining = remaining.clone();
        let timeout_id = timeout_id.clone();
        let close_fn = close_fn.clone();
        let window = window.clone();
        move || {
            let ms = remaining.get();
            let callback = Closure::<dyn Fn()>::new({
                let close_fn = close_fn.clone();
                move || {
                    close_fn();
                }
            });
            let id = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    ms as i32,
                )
                .unwrap_or(-1);
            timeout_id.set(id);
            callback.forget();
        }
    };

    // Initial schedule.
    schedule();

    // mouseenter: pause timer.
    {
        let remaining = remaining.clone();
        let start_time = start_time.clone();
        let timeout_id = timeout_id.clone();
        let window = window.clone();
        let on_mouseenter = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
            let elapsed = js_sys::Date::now() - start_time.get();
            let remaining_after_pause = (remaining.get() - elapsed).max(0.0);
            remaining.set(remaining_after_pause);
            // Clear existing timeout.
            if timeout_id.get() >= 0 {
                window.clear_timeout_with_handle(timeout_id.get());
                timeout_id.set(-1);
            }
        });
        let target: &web_sys::EventTarget = toast_el.as_ref();
        target
            .add_event_listener_with_callback("mouseenter", on_mouseenter.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_mouseenter.forget();
    }

    // mouseleave: resume timer.
    {
        let start_time = start_time.clone();
        let on_mouseleave = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
            start_time.set(js_sys::Date::now());
            schedule();
        });
        let target: &web_sys::EventTarget = toast_el.as_ref();
        target
            .add_event_listener_with_callback("mouseleave", on_mouseleave.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_mouseleave.forget();
    }

    // [data-toast-close] click.
    if let Ok(Some(close_btn)) = toast_el.query_selector("[data-toast-close]") {
        let close_fn = close_fn.clone();
        let on_close = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
            close_fn();
        });
        let btn_target: &web_sys::EventTarget = close_btn.as_ref();
        btn_target
            .add_event_listener_with_callback("click", on_close.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_close.forget();
    }
}

fn dismiss_toast(toast_el: &Element) {
    let _ = toast_el.set_attribute("data-state", "closed");
    // Remove from DOM after a short transition window (300ms).
    let toast_c = toast_el.clone();
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let callback = Closure::<dyn Fn()>::new(move || {
        if let Some(parent) = toast_c.parent_node() {
            let _ = parent.remove_child(toast_c.as_ref());
        }
    });
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        300,
    );
    callback.forget();
}

fn enforce_max(toaster: &Element) {
    let toasts = match toaster.query_selector_all("[data-toast]") {
        Ok(nl) => nl,
        Err(_) => return,
    };
    // Remove oldest (last children) if at or above limit.
    while toasts.length() as usize >= MAX_VISIBLE {
        if let Some(last) = toaster.last_child() {
            let _ = toaster.remove_child(&last);
        } else {
            break;
        }
    }
}

fn wire_existing_toasts(toaster: &Element) {
    let node_list = match toaster.query_selector_all("[data-toast]") {
        Ok(node_list) => node_list,
        Err(_) => return,
    };
    for i in 0..node_list.length() {
        if let Some(n) = node_list.item(i)
            && let Ok(el) = n.dyn_into::<Element>()
        {
            let duration = el
                .get_attribute("data-duration")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(DEFAULT_DURATION_MS);
            wire_toast(&el, duration);
        }
    }
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn dispatch_initialized(el: &Element, name: &str) {
    use web_sys::CustomEventInit;
    let init = CustomEventInit::new();
    let detail = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &detail,
        &JsValue::from_str("name"),
        &JsValue::from_str(name),
    );
    init.set_detail(&detail);
    init.set_bubbles(true);
    if let Ok(ev) = CustomEvent::new_with_event_init_dict("basecoat:initialized", &init) {
        let target: &web_sys::EventTarget = el.as_ref();
        let _ = target.dispatch_event(&ev);
    }
}
