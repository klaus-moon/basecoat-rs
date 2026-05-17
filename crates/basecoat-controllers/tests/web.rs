// Integration tests for basecoat-controllers.
//
// Run with:
//   wasm-pack test --headless --chrome crates/basecoat-controllers
// or:
//   wasm-pack test --headless --firefox crates/basecoat-controllers

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

// ---------------------------------------------------------------------------
// Dialog tests
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn dialog_opens_on_trigger_click() {
    let doc = document();

    // Build minimal dialog structure.
    let container = doc.create_element("div").unwrap();
    container
        .set_attribute("data-basecoat-hydrate", "dialog")
        .unwrap();
    container
        .set_attribute("data-basecoat-version", "0.1")
        .unwrap();

    let trigger = doc.create_element("button").unwrap();
    trigger.set_attribute("data-dialog-trigger", "").unwrap();
    trigger
        .set_attribute("aria-controls", "test-dialog-1")
        .unwrap();

    let dialog = doc.create_element("dialog").unwrap();
    dialog.set_id("test-dialog-1");
    dialog.set_inner_html("<button>Close</button>");

    container.append_child(&trigger).unwrap();
    container.append_child(&dialog).unwrap();
    doc.body().unwrap().append_child(&container).unwrap();

    // Attach controller.
    basecoat_controllers::controllers::dialog::attach(container.clone());

    // Simulate trigger click.
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    let click = MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap();
    trigger.dispatch_event(&click).unwrap();

    // dialog.open should be true after showModal().
    let dialog_el = dialog.dyn_ref::<web_sys::HtmlDialogElement>().unwrap();
    assert!(
        dialog_el.open(),
        "dialog should be open after trigger click"
    );
    assert_eq!(
        dialog.get_attribute("data-state").as_deref(),
        Some("open"),
        "data-state should be 'open'"
    );

    // Clean up.
    doc.body().unwrap().remove_child(&container).unwrap();
}

#[wasm_bindgen_test]
fn dialog_closes_on_programmatic_close() {
    let doc = document();

    let container = doc.create_element("div").unwrap();
    container
        .set_attribute("data-basecoat-hydrate", "dialog")
        .unwrap();
    container
        .set_attribute("data-basecoat-version", "0.1")
        .unwrap();

    let trigger = doc.create_element("button").unwrap();
    trigger.set_attribute("data-dialog-trigger", "").unwrap();
    trigger
        .set_attribute("aria-controls", "test-dialog-2")
        .unwrap();

    let dialog = doc.create_element("dialog").unwrap();
    dialog.set_id("test-dialog-2");
    dialog.set_inner_html("<button>Close</button>");

    container.append_child(&trigger).unwrap();
    container.append_child(&dialog).unwrap();
    doc.body().unwrap().append_child(&container).unwrap();

    basecoat_controllers::controllers::dialog::attach(container.clone());

    // Open via trigger click.
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    let click = MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap();
    trigger.dispatch_event(&click).unwrap();

    let dialog_el = dialog.dyn_ref::<web_sys::HtmlDialogElement>().unwrap();
    assert!(dialog_el.open());

    // Close programmatically (simulates Escape which browser fires "close" event).
    dialog_el.close();
    // Dispatch the "close" event manually since we're in a test environment.
    use web_sys::{Event, EventInit};
    let einit = EventInit::new();
    einit.set_bubbles(false);
    let close_ev = Event::new_with_event_init_dict("close", &einit).unwrap();
    let target: &web_sys::EventTarget = dialog.as_ref();
    target.dispatch_event(&close_ev).unwrap();

    assert_eq!(
        dialog.get_attribute("data-state").as_deref(),
        Some("closed"),
        "data-state should be 'closed' after close"
    );

    doc.body().unwrap().remove_child(&container).unwrap();
}

// ---------------------------------------------------------------------------
// Tabs tests
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
fn tabs_activates_second_tab_on_click() {
    let doc = document();

    let container = doc.create_element("div").unwrap();
    container
        .set_attribute("data-basecoat-hydrate", "tabs")
        .unwrap();
    container
        .set_attribute("data-basecoat-version", "0.1")
        .unwrap();

    // Build tablist + 2 tabs + 2 panels.
    let tablist = doc.create_element("div").unwrap();
    tablist.set_attribute("role", "tablist").unwrap();

    let tab1 = doc.create_element("button").unwrap();
    tab1.set_attribute("role", "tab").unwrap();
    tab1.set_attribute("aria-controls", "panel-1").unwrap();
    tab1.set_id("tab-1");
    tab1.set_inner_html("Tab 1");

    let tab2 = doc.create_element("button").unwrap();
    tab2.set_attribute("role", "tab").unwrap();
    tab2.set_attribute("aria-controls", "panel-2").unwrap();
    tab2.set_id("tab-2");
    tab2.set_inner_html("Tab 2");

    tablist.append_child(&tab1).unwrap();
    tablist.append_child(&tab2).unwrap();

    let panel1 = doc.create_element("div").unwrap();
    panel1.set_attribute("role", "tabpanel").unwrap();
    panel1.set_id("panel-1");
    panel1.set_attribute("aria-labelledby", "tab-1").unwrap();

    let panel2 = doc.create_element("div").unwrap();
    panel2.set_attribute("role", "tabpanel").unwrap();
    panel2.set_id("panel-2");
    panel2.set_attribute("aria-labelledby", "tab-2").unwrap();

    container.append_child(&tablist).unwrap();
    container.append_child(&panel1).unwrap();
    container.append_child(&panel2).unwrap();

    doc.body().unwrap().append_child(&container).unwrap();

    basecoat_controllers::controllers::tabs::attach(container.clone());

    // Initially tab1 should be active.
    assert_eq!(
        tab1.get_attribute("aria-selected").as_deref(),
        Some("true"),
        "tab1 should be selected initially"
    );
    assert!(
        panel2.get_attribute("hidden").is_some(),
        "panel2 should be hidden initially"
    );

    // Click tab2.
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    let click = MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap();
    tab2.dispatch_event(&click).unwrap();

    assert_eq!(
        tab2.get_attribute("aria-selected").as_deref(),
        Some("true"),
        "tab2 should be selected after click"
    );
    assert_eq!(
        tab1.get_attribute("aria-selected").as_deref(),
        Some("false"),
        "tab1 should be deselected after tab2 click"
    );
    assert!(
        panel2.get_attribute("hidden").is_none(),
        "panel2 should be visible after clicking tab2"
    );
    assert!(
        panel1.get_attribute("hidden").is_some(),
        "panel1 should be hidden after clicking tab2"
    );

    doc.body().unwrap().remove_child(&container).unwrap();
}

// ---------------------------------------------------------------------------
// Toast tests
// ---------------------------------------------------------------------------

#[wasm_bindgen_test]
async fn toast_appears_programmatically() {
    use js_sys::Promise;
    use wasm_bindgen_futures::JsFuture;

    let doc = document();

    let toaster = doc.create_element("div").unwrap();
    toaster.set_attribute("data-toaster", "").unwrap();
    toaster
        .set_attribute("data-basecoat-hydrate", "toast")
        .unwrap();
    toaster
        .set_attribute("data-basecoat-version", "0.1")
        .unwrap();

    doc.body().unwrap().append_child(&toaster).unwrap();

    // Attach controller (wires existing toasts + exposes API).
    basecoat_controllers::controllers::toast::attach(toaster.clone());

    // Call window.basecoat.toast({...}) via JS.
    let window = web_sys::window().unwrap();
    let basecoat_obj = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("basecoat"))
        .expect("window.basecoat not found");
    assert!(
        !basecoat_obj.is_undefined(),
        "window.basecoat should be defined after hydrate"
    );

    // Expose API manually (hydrate() wasn't called in this isolated test).
    let obj = js_sys::Object::new();
    basecoat_controllers::controllers::toast::expose_toast_api(&obj);

    let toast_fn = js_sys::Reflect::get(&obj, &wasm_bindgen::JsValue::from_str("toast")).unwrap();
    let toast_fn = toast_fn.dyn_into::<js_sys::Function>().unwrap();

    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &opts,
        &wasm_bindgen::JsValue::from_str("title"),
        &wasm_bindgen::JsValue::from_str("Test toast"),
    );
    let _ = js_sys::Reflect::set(
        &opts,
        &wasm_bindgen::JsValue::from_str("duration"),
        &wasm_bindgen::JsValue::from_f64(200.0),
    );
    toast_fn
        .call1(&wasm_bindgen::JsValue::UNDEFINED, &opts)
        .unwrap();

    // Toast should be present immediately.
    let toasts = toaster.query_selector_all("[data-toast]").unwrap();
    assert_eq!(toasts.length(), 1, "one toast should be present");

    // Wait past duration + transition (200 + 300 + 50ms buffer = 550ms).
    let promise = Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 550)
            .unwrap();
    });
    JsFuture::from(promise).await.unwrap();

    let toasts_after = toaster.query_selector_all("[data-toast]").unwrap();
    assert_eq!(
        toasts_after.length(),
        0,
        "toast should be removed after duration elapses"
    );

    doc.body().unwrap().remove_child(&toaster).unwrap();
}
