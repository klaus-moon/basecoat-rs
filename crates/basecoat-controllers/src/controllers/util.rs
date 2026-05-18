// Shared controller utilities.
//
// Small helpers reused across multiple controllers. Each helper is documented
// inline; keep this module narrow — anything controller-specific belongs in
// the controller module itself.

use wasm_bindgen::JsValue;
use web_sys::{CustomEvent, CustomEventInit, Element};

/// Fire a `basecoat:initialized` CustomEvent on `el`.
///
/// `detail.name` carries the controller name so listeners can disambiguate
/// (e.g. `"dialog"`, `"tabs"`, `"toast"`). The event bubbles.
///
/// Centralized here so dialog/tabs/toast (and future controllers) share the
/// exact same payload shape — listeners can rely on `event.detail.name` being
/// present regardless of source controller.
pub fn dispatch_initialized(el: &Element, name: &str) {
    let init = CustomEventInit::new();
    let detail = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &detail,
        &JsValue::from_str("name"),
        &JsValue::from_str(name),
    );
    init.set_detail(&detail);
    init.set_bubbles(true);
    let Ok(ev) = CustomEvent::new_with_event_init_dict("basecoat:initialized", &init) else {
        return;
    };
    let target: &web_sys::EventTarget = el.as_ref();
    let _ = target.dispatch_event(&ev);
}
