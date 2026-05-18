// Select controller
//
// Wires a `.select` wrapper composed of:
//   - a hidden native <select data-select-native> (form source of truth)
//   - a visible trigger <button data-select-trigger aria-haspopup="listbox">
//   - a floating listbox <div role="listbox" data-select-listbox> containing
//     <button role="option" data-value="..."> items
//
// Behaviour:
//   - Trigger click toggles the listbox open/closed.
//   - Open positions the listbox under the trigger via @floating-ui/dom.
//   - Open width-matches the listbox min-width to the trigger.
//   - Open registers Roving tabindex (Vertical, wrap, type-ahead) on options.
//   - Open registers dismiss listeners (click-outside, Escape, focus-out).
//   - Option click: update hidden <select>.value, dispatch 'change' on the
//     native <select>, update trigger label, mark aria-selected, close.
//   - data-state="open"|"closed" mirrored on the wrapper for CSS.
//   - data-select-initialized flag toggled on the wrapper after attach so
//     the CSS hover-fallback style stops applying.
//
// Closure lifetime mix: trigger/option/click closures persist for the
// element's lifetime (Closure::forget — same rationale as dialog.rs). The
// per-open RovingHandle and DismissHandle are kept inside an Rc<RefCell>
// shared between every closure, and dropped when the listbox closes.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    CustomEvent, CustomEventInit, Element, HtmlButtonElement, HtmlElement, HtmlSelectElement,
    MouseEvent,
};

use super::dismiss::{self, DismissHandle};
use super::floating;
use super::keyboard::{self, Orientation, RovingHandle, RovingOpts};
use super::util::dispatch_initialized;

/// Per-instance state retained across the trigger/option event listeners.
struct SelectState {
    wrapper: HtmlElement,
    trigger: HtmlButtonElement,
    listbox: HtmlElement,
    native: HtmlSelectElement,
    options: Vec<HtmlButtonElement>,
    open: bool,
    roving: Option<RovingHandle>,
    dismiss: Option<DismissHandle>,
}

pub fn attach(root: Element) {
    let wrapper = match root.dyn_into::<HtmlElement>() {
        Ok(el) => el,
        Err(_) => return,
    };

    let trigger = match find_child::<HtmlButtonElement>(&wrapper, "[data-select-trigger]") {
        Some(el) => el,
        None => {
            web_sys::console::warn_1(
                &"[basecoat:select] no [data-select-trigger] found".into(),
            );
            return;
        }
    };

    let listbox = match find_child::<HtmlElement>(&wrapper, "[data-select-listbox]") {
        Some(el) => el,
        None => {
            web_sys::console::warn_1(
                &"[basecoat:select] no [data-select-listbox] found".into(),
            );
            return;
        }
    };

    let native = match find_child::<HtmlSelectElement>(&wrapper, "[data-select-native]") {
        Some(el) => el,
        None => {
            web_sys::console::warn_1(
                &"[basecoat:select] no [data-select-native] found".into(),
            );
            return;
        }
    };

    let options = collect_options(&listbox);
    if options.is_empty() {
        web_sys::console::warn_1(&"[basecoat:select] no [role='option'] items found".into());
    }

    // Initial state sync — make the trigger label match the native value.
    sync_trigger_label(&trigger, &native, &options);
    set_state(wrapper.as_ref(), "closed");

    let state = Rc::new(RefCell::new(SelectState {
        wrapper: wrapper.clone(),
        trigger: trigger.clone(),
        listbox: listbox.clone(),
        native: native.clone(),
        options: options.clone(),
        open: false,
        roving: None,
        dismiss: None,
    }));

    // Trigger click → toggle.
    {
        let state_c = state.clone();
        let on_click = Closure::<dyn Fn(MouseEvent)>::new(move |e: MouseEvent| {
            e.prevent_default();
            toggle(&state_c);
        });
        let target: &web_sys::EventTarget = trigger.as_ref();
        let _ = target
            .add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
        on_click.forget();
    }

    // Option click → select.
    for option in &options {
        let state_c = state.clone();
        let value = option.get_attribute("data-value").unwrap_or_default();
        let on_click = Closure::<dyn Fn(MouseEvent)>::new(move |e: MouseEvent| {
            e.prevent_default();
            select_value(&state_c, &value);
        });
        let target: &web_sys::EventTarget = option.as_ref();
        let _ = target
            .add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
        on_click.forget();
    }

    // Mark initialized so the CSS hover-fallback style stops applying.
    let _ = wrapper.set_attribute("data-select-initialized", "");

    dispatch_initialized(wrapper.as_ref(), "select");
}

// ---------------------------------------------------------------------------
// Open / close
// ---------------------------------------------------------------------------

fn toggle(state: &Rc<RefCell<SelectState>>) {
    let is_open = state.borrow().open;
    if is_open {
        close(state);
    } else {
        open(state);
    }
}

fn open(state: &Rc<RefCell<SelectState>>) {
    {
        let mut s = state.borrow_mut();
        if s.open {
            return;
        }
        s.open = true;

        // Reveal listbox.
        let _ = s.listbox.remove_attribute("hidden");
        let _ = s.trigger.set_attribute("aria-expanded", "true");
        set_state(s.wrapper.as_ref(), "open");

        // Width-match the listbox to the trigger. We merge into any
        // existing inline style so the floating-UI shim's `position`,
        // `left`, and `top` declarations are preserved.
        let trigger_width = s.trigger.client_width();
        if trigger_width > 0 {
            let listbox_el: &Element = s.listbox.as_ref();
            let existing = listbox_el
                .get_attribute("style")
                .unwrap_or_default();
            let new_style = merge_min_width(&existing, trigger_width);
            let _ = listbox_el.set_attribute("style", &new_style);
        }

        // Wire dismiss listeners.
        let state_for_dismiss = state.clone();
        let on_dismiss = Box::new(move || {
            close(&state_for_dismiss);
        });
        let trigger_as_html: HtmlElement = s.trigger.clone().into();
        let handle = dismiss::attach(s.listbox.clone(), trigger_as_html, on_dismiss);
        s.dismiss = Some(handle);

        // Wire roving tabindex on the options.
        let opts_iter = s
            .options
            .iter()
            .cloned()
            .map(HtmlElement::from)
            .collect::<Vec<_>>();
        let listbox_el: &Element = s.listbox.as_ref();
        let roving = keyboard::attach(
            listbox_el,
            opts_iter,
            RovingOpts {
                orientation: Orientation::Vertical,
                wrap: true,
                type_ahead: true,
            },
        );
        s.roving = Some(roving);
    }

    // Compute floating-UI position. The state borrow is dropped before
    // awaiting so concurrent closures can still read state.
    let trigger = state.borrow().trigger.clone();
    let listbox = state.borrow().listbox.clone();
    let trigger_el: Element = trigger.into();
    wasm_bindgen_futures::spawn_local(async move {
        floating::position(&trigger_el, &listbox, "bottom-start").await;
    });

    // Focus the currently-selected option, else the first enabled option.
    focus_initial(state);
}

fn close(state: &Rc<RefCell<SelectState>>) {
    let mut s = state.borrow_mut();
    if !s.open {
        return;
    }
    s.open = false;

    // Drop the dismiss/roving handles so their listeners detach.
    s.dismiss.take();
    s.roving.take();

    let _ = s.listbox.set_attribute("hidden", "");
    let _ = s.trigger.set_attribute("aria-expanded", "false");
    set_state(s.wrapper.as_ref(), "closed");

    // Return focus to the trigger.
    let _ = s.trigger.focus();
}

// ---------------------------------------------------------------------------
// Selection
// ---------------------------------------------------------------------------

fn select_value(state: &Rc<RefCell<SelectState>>, value: &str) {
    {
        let s = state.borrow();
        s.native.set_value(value);

        // aria-selected sync on options.
        for option in &s.options {
            let opt_value = option.get_attribute("data-value").unwrap_or_default();
            let selected = if opt_value == value { "true" } else { "false" };
            let _ = option.set_attribute("aria-selected", selected);
        }

        // Update the trigger label.
        sync_trigger_label(&s.trigger, &s.native, &s.options);

        // Dispatch native 'change' event for form integration.
        dispatch_change(&s.native);
    }
    close(state);
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn find_child<T>(parent: &HtmlElement, selector: &str) -> Option<T>
where
    T: JsCast,
{
    let parent_el: &Element = parent.as_ref();
    let node = parent_el.query_selector(selector).ok().flatten()?;
    node.dyn_into::<T>().ok()
}

fn collect_options(listbox: &HtmlElement) -> Vec<HtmlButtonElement> {
    let listbox_el: &Element = listbox.as_ref();
    let node_list = match listbox_el.query_selector_all("[role='option']") {
        Ok(nl) => nl,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::with_capacity(node_list.length() as usize);
    for i in 0..node_list.length() {
        if let Some(n) = node_list.item(i)
            && let Ok(el) = n.dyn_into::<HtmlButtonElement>()
        {
            out.push(el);
        }
    }
    out
}

fn sync_trigger_label(
    trigger: &HtmlButtonElement,
    native: &HtmlSelectElement,
    options: &[HtmlButtonElement],
) {
    let value = native.value();
    let label = options
        .iter()
        .find(|o| o.get_attribute("data-value").as_deref() == Some(value.as_str()))
        .map(|o| o.text_content().unwrap_or_default());

    let trigger_el: &Element = trigger.as_ref();
    let value_span = trigger_el
        .query_selector("[data-select-value]")
        .ok()
        .flatten();
    if let Some(span) = value_span {
        if let Some(text) = label {
            span.set_text_content(Some(&text));
        }
    } else if let Some(text) = label {
        trigger.set_text_content(Some(&text));
    }
}

fn focus_initial(state: &Rc<RefCell<SelectState>>) {
    let s = state.borrow();
    let target = s
        .options
        .iter()
        .find(|o| o.get_attribute("aria-selected").as_deref() == Some("true"))
        .or_else(|| s.options.iter().find(|o| !is_disabled(o)));
    if let Some(opt) = target {
        let _ = opt.focus();
    }
}

fn is_disabled(opt: &HtmlButtonElement) -> bool {
    if opt.disabled() {
        return true;
    }
    opt.get_attribute("aria-disabled").as_deref() == Some("true")
}

fn set_state(el: &Element, state: &str) {
    let _ = el.set_attribute("data-state", state);
}

/// Merge a `min-width: {width}px;` declaration into an existing inline style
/// string, dropping any prior `min-width` so we don't accumulate stale values
/// across opens.
fn merge_min_width(existing: &str, width: i32) -> String {
    let mut out = String::with_capacity(existing.len() + 24);
    for part in existing.split(';') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let key = trimmed
            .split(':')
            .next()
            .map(|s| s.trim().to_ascii_lowercase())
            .unwrap_or_default();
        if key == "min-width" {
            continue;
        }
        out.push_str(trimmed);
        out.push_str("; ");
    }
    out.push_str(&format!("min-width: {width}px;"));
    out
}

fn dispatch_change(native: &HtmlSelectElement) {
    let init = CustomEventInit::new();
    init.set_bubbles(true);
    let Ok(ev) = CustomEvent::new_with_event_init_dict("change", &init) else {
        return;
    };
    let target: &web_sys::EventTarget = native.as_ref();
    let _ = target.dispatch_event(&ev);
}
