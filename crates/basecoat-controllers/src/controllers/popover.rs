// Popover controller
//
// Wires a `<details class="popover">` element via Phase 1 primitives:
//   - `floating::compute_position` for trigger->content positioning
//   - `floating::auto_update` to keep position synced while open
//   - `dismiss::attach` for click-outside / Escape / focus-out
//   - `data-state="open"|"closed"` mirroring + `aria-expanded` sync
//   - CustomEvent("basecoat:initialized") after wiring
//
// Closure lifetime strategy:
//   The `toggle` and `summary click` closures fire repeatedly across the
//   popover's lifetime, so they live in a thread-local registry keyed by
//   element id. The dismiss handle's listeners, by contrast, are owned
//   per-open-session: we keep them inside `Rc<RefCell<...>>` slots and drop
//   them on close, which removes the document listeners (see dismiss.rs).
//
//   This split mirrors what's documented in dismiss.rs: dynamically-attaching
//   listeners must be Drop'd, page-lifetime listeners can leak.
//
// `HtmlDetailsElement` is not in the `web-sys` feature set for this crate
// (Cargo.toml is owned by Phase 4), so we operate on the root as a plain
// `Element` and use `set_attribute("open", ...)` / `has_attribute("open")` to
// drive the `<details>` open state. This works because the DOM mirrors the
// reflected attribute synchronously.
//
// Arrow positioning (v0.2 decision):
//   The optional `<div data-popover-arrow>` is rendered by the components and
//   leptos layers when callers opt in. The controller does NOT call
//   floating-ui's `arrow` middleware in v0.2 — the visual arrow stays
//   CSS-anchored at the content panel's edge. Adding the middleware bridge
//   requires extending the JS shim; we defer that to v0.3 to keep the v0.2
//   surface narrow.

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, MouseEvent};

use super::dismiss::{self, DismissHandle};
use super::floating;
use super::util::dispatch_initialized;

/// Per-popover state held in a thread-local registry. We keep the long-lived
/// closures here so they outlive `attach()`.
struct PopoverState {
    /// Stored to keep the toggle-listener closure alive for the page lifetime.
    _toggle: Closure<dyn Fn()>,
    /// Stored to keep the summary-click stopPropagation closure alive.
    _summary_click: Closure<dyn Fn(MouseEvent)>,
}

thread_local! {
    /// Page-lifetime store of every wired popover. Indexed by element id; the
    /// id is required by the DOM contract so collisions are a caller bug.
    static POPOVERS: RefCell<Vec<(String, PopoverState)>> = const { RefCell::new(Vec::new()) };
}

fn store_state(id: String, state: PopoverState) {
    POPOVERS.with(|cell| cell.borrow_mut().push((id, state)));
}

pub fn attach(root: Element) {
    if !root.tag_name().eq_ignore_ascii_case("details") {
        web_sys::console::warn_1(
            &"[basecoat:popover] root is not a <details> element".into(),
        );
        return;
    }

    let id = root.id();
    if id.is_empty() {
        web_sys::console::warn_1(
            &"[basecoat:popover] <details data-basecoat-hydrate> requires an id".into(),
        );
        return;
    }

    // Locate trigger (<summary>) and content (<div role="dialog">).
    let summary: HtmlElement = match root.query_selector("summary") {
        Ok(Some(el)) => match el.dyn_into::<HtmlElement>() {
            Ok(e) => e,
            Err(_) => return,
        },
        _ => {
            web_sys::console::warn_1(
                &format!("[basecoat:popover] #{id} missing <summary>").into(),
            );
            return;
        }
    };

    let content: HtmlElement = match root.query_selector("div[role=\"dialog\"]") {
        Ok(Some(el)) => match el.dyn_into::<HtmlElement>() {
            Ok(e) => e,
            Err(_) => return,
        },
        _ => {
            web_sys::console::warn_1(
                &format!("[basecoat:popover] #{id} missing div[role=\"dialog\"]").into(),
            );
            return;
        }
    };

    // Wire aria-controls -> trigger so the dismiss helper and a11y tooling can
    // resolve the pair without depending on tree structure.
    let _ = summary.set_attribute("aria-controls", &format!("{id}-content"));

    set_state(&root, false);

    // Per-open-session state: the active DismissHandle and the autoUpdate
    // cleanup function. Held inside `Rc<RefCell<...>>` so both the toggle
    // closure and the dismiss callback can take/replace them. Cleared on close.
    let dismiss_handle: Rc<RefCell<Option<DismissHandle>>> = Rc::new(RefCell::new(None));
    let auto_update_cleanup: Rc<RefCell<Option<js_sys::Function>>> =
        Rc::new(RefCell::new(None));

    // ---- `toggle` event on <details>: sync data-state, aria-expanded, ------
    //      position the content, and attach/detach dismiss listeners.
    let toggle = {
        let root_c = root.clone();
        let summary_c = summary.clone();
        let content_c = content.clone();
        let dismiss_c = dismiss_handle.clone();
        let auto_update_c = auto_update_cleanup.clone();
        Closure::<dyn Fn()>::new(move || {
            let is_open = root_c.has_attribute("open");
            set_state(&root_c, is_open);
            let _ = summary_c.set_attribute(
                "aria-expanded",
                if is_open { "true" } else { "false" },
            );

            if is_open {
                open_popover(
                    &root_c,
                    &summary_c,
                    &content_c,
                    &dismiss_c,
                    &auto_update_c,
                );
            } else {
                close_popover(&dismiss_c, &auto_update_c);
            }
        })
    };
    let toggle_target: &web_sys::EventTarget = root.as_ref();
    let _ = toggle_target
        .add_event_listener_with_callback("toggle", toggle.as_ref().unchecked_ref());

    // ---- Stop click events on the <summary> from bubbling out of the popover.
    //      We intentionally do NOT preventDefault — the native `<details>`
    //      toggle is the whole point — we only stop event-propagation crossing
    //      the popover boundary.
    let summary_click = Closure::<dyn Fn(MouseEvent)>::new(move |e: MouseEvent| {
        e.stop_propagation();
    });
    let summary_target: &web_sys::EventTarget = summary.as_ref();
    let _ = summary_target
        .add_event_listener_with_callback("click", summary_click.as_ref().unchecked_ref());

    // ---- Persist closures so they outlive `attach()`. -----------------------
    store_state(
        id,
        PopoverState {
            _toggle: toggle,
            _summary_click: summary_click,
        },
    );

    dispatch_initialized(&root, "popover");
}

fn open_popover(
    root: &Element,
    summary: &HtmlElement,
    content: &HtmlElement,
    dismiss_slot: &Rc<RefCell<Option<DismissHandle>>>,
    auto_update_slot: &Rc<RefCell<Option<js_sys::Function>>>,
) {
    let placement = root
        .get_attribute("data-placement")
        .unwrap_or_else(|| "bottom".to_string());
    let offset = root
        .get_attribute("data-offset")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(8.0);

    // Kick off initial positioning. The JS shim mutates content.style itself.
    let initial = floating::compute_position(summary.as_ref(), content, &placement, offset);
    let _ = wasm_bindgen_futures::future_to_promise(async move {
        let _ = wasm_bindgen_futures::JsFuture::from(initial).await;
        Ok(JsValue::UNDEFINED)
    });

    // Keep position synced while open via `autoUpdate`.
    let auto_update_cb = {
        let summary_c = summary.clone();
        let content_c = content.clone();
        let placement_c = placement.clone();
        Closure::<dyn Fn()>::new(move || {
            let _ = floating::compute_position(
                summary_c.as_ref(),
                &content_c,
                &placement_c,
                offset,
            );
        })
    };
    let cleanup = floating::auto_update(
        summary.as_ref(),
        content,
        auto_update_cb.as_ref().unchecked_ref(),
    );
    // The callback fires asynchronously from autoUpdate; we must outlive the
    // function scope. We `forget()` here because autoUpdate's teardown does
    // not give us a way to drop the closure with it. See dismiss.rs for the
    // owned-handle pattern used elsewhere; the arrow-middleware bridge in
    // v0.3 may need to revisit this.
    auto_update_cb.forget();
    *auto_update_slot.borrow_mut() = Some(cleanup);

    // Attach click-outside / Escape / focus-out dismiss listeners.
    let root_for_dismiss = root.clone();
    let handle = dismiss::attach(
        content.clone(),
        summary.clone(),
        Box::new(move || {
            // Removing the `open` attribute on <details> fires the `toggle`
            // event which then clears the dismiss slot via `close_popover`.
            let _ = root_for_dismiss.remove_attribute("open");
        }),
    );
    *dismiss_slot.borrow_mut() = Some(handle);
}

fn close_popover(
    dismiss_slot: &Rc<RefCell<Option<DismissHandle>>>,
    auto_update_slot: &Rc<RefCell<Option<js_sys::Function>>>,
) {
    // Dropping the DismissHandle removes its document listeners.
    *dismiss_slot.borrow_mut() = None;
    // Call the autoUpdate teardown if we have one.
    if let Some(cleanup) = auto_update_slot.borrow_mut().take() {
        let _ = cleanup.call0(&JsValue::UNDEFINED);
    }
}

fn set_state(el: &Element, is_open: bool) {
    let state = if is_open { "open" } else { "closed" };
    let _ = el.set_attribute("data-state", state);
}
