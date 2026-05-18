// Tabs controller
//
// Wires a tablist/tab/tabpanel pattern:
//   - Container has role="tablist"; tabs have role="tab"; panels have role="tabpanel".
//   - Trigger's aria-controls → panel id; panel's aria-labelledby → trigger id.
//   - Click tab → activate (aria-selected, data-state, hidden).
//   - Arrow keys (respecting aria-orientation), Home/End navigation.
//   - Roving tabindex: active tab tabindex="0", others "-1".
//   - Dispatch basecoat:initialized after wiring.
//
// Keyboard navigation is delegated to `super::keyboard::RovingTabindex`; the
// `RovingHandle` is intentionally leaked because the tablist (like the
// dialog) lives for the lifetime of the page.
//
// Closure lifetime: Closure::forget() — same rationale as dialog.rs.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, MouseEvent};

use super::keyboard::{self, Orientation, RovingOpts};
use super::util::dispatch_initialized;

pub fn attach(root: Element) {
    let tablist_node = match root.query_selector("[role='tablist']") {
        Ok(Some(el)) => el,
        _ => {
            // The root itself may be the tablist.
            if root.get_attribute("role").as_deref() == Some("tablist") {
                root.clone()
            } else {
                web_sys::console::warn_1(&"[basecoat:tabs] no [role=tablist] found".into());
                return;
            }
        }
    };

    let tabs = collect_role(&tablist_node, "tab");
    if tabs.is_empty() {
        web_sys::console::warn_1(&"[basecoat:tabs] no [role=tab] found".into());
        return;
    }

    // Initial state: activate first selected tab, or first tab.
    let initial = tabs
        .iter()
        .position(|t| t.get_attribute("aria-selected").as_deref() == Some("true"))
        .unwrap_or(0);
    activate_tab(&root, &tabs, initial);

    // Wire click on each tab.
    for (idx, tab) in tabs.iter().enumerate() {
        let root_c = root.clone();
        let tabs_c = tabs.clone();
        let on_click = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
            activate_tab(&root_c, &tabs_c, idx);
        });
        tab.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_click.forget();
    }

    // Wire arrow/Home/End navigation via the shared RovingTabindex helper.
    // We additionally hook a focusin listener on each tab so the panel
    // switches to whatever tab the helper just focused. The RovingHandle is
    // leaked because tabs share the page-lifetime listener pattern.
    let orientation = if tablist_node
        .get_attribute("aria-orientation")
        .as_deref()
        == Some("vertical")
    {
        Orientation::Vertical
    } else {
        Orientation::Horizontal
    };

    let handle = keyboard::attach(
        &tablist_node,
        tabs.clone(),
        RovingOpts {
            orientation,
            wrap: true,
            type_ahead: false,
        },
    );
    // Intentionally leak: tabs live for the lifetime of the page.
    std::mem::forget(handle);

    // Focus-driven activation: when a tab gains focus (e.g. via the roving
    // helper's ArrowRight), activate it.
    let active_idx = Rc::new(RefCell::new(initial));
    for (idx, tab) in tabs.iter().enumerate() {
        let root_c = root.clone();
        let tabs_c = tabs.clone();
        let active_idx_c = active_idx.clone();
        let on_focus = Closure::<dyn Fn(web_sys::FocusEvent)>::new(
            move |_e: web_sys::FocusEvent| {
                if *active_idx_c.borrow() == idx {
                    return;
                }
                *active_idx_c.borrow_mut() = idx;
                activate_tab(&root_c, &tabs_c, idx);
            },
        );
        let target: &web_sys::EventTarget = tab.as_ref();
        target
            .add_event_listener_with_callback("focus", on_focus.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_focus.forget();
    }

    dispatch_initialized(&root, "tabs");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn collect_role(container: &Element, role: &str) -> Vec<HtmlElement> {
    let selector = format!("[role='{role}']");
    let node_list = match container.query_selector_all(&selector) {
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

fn activate_tab(root: &Element, tabs: &[HtmlElement], idx: usize) {
    // Update tabs.
    for (i, tab) in tabs.iter().enumerate() {
        let active = i == idx;
        let _ = tab.set_attribute("aria-selected", if active { "true" } else { "false" });
        let _ = tab.set_attribute("tabindex", if active { "0" } else { "-1" });
        let _ = tab.set_attribute("data-state", if active { "active" } else { "inactive" });
    }

    // Update panels.
    let panels = collect_role(root, "tabpanel");
    let active_tab = &tabs[idx];
    let active_controls = active_tab.get_attribute("aria-controls");

    for panel in &panels {
        let panel_el: &Element = panel.as_ref();
        let is_active = active_controls
            .as_ref()
            .map(|id| panel_el.id() == *id)
            .unwrap_or(false);
        if is_active {
            let _ = panel_el.remove_attribute("hidden");
        } else {
            let _ = panel_el.set_attribute("hidden", "");
        }
    }
}
