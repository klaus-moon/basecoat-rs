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
// Closure lifetime: Closure::forget() — same rationale as dialog.rs.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent};

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

    // Wire keydown on the tablist for arrow/home/end navigation.
    {
        let root_c = root.clone();
        let tablist_c = tablist_node.clone();
        let tabs_c = tabs.clone();
        let on_keydown = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            let orientation = tablist_c
                .get_attribute("aria-orientation")
                .unwrap_or_else(|| "horizontal".to_string());
            let horizontal = orientation != "vertical";

            let current = current_index(&tabs_c);
            let len = tabs_c.len();

            let next = match e.key().as_str() {
                "ArrowRight" if horizontal => Some((current + 1) % len),
                "ArrowLeft" if horizontal => Some(if current == 0 { len - 1 } else { current - 1 }),
                "ArrowDown" if !horizontal => Some((current + 1) % len),
                "ArrowUp" if !horizontal => Some(if current == 0 { len - 1 } else { current - 1 }),
                "Home" => Some(0),
                "End" => Some(len - 1),
                _ => None,
            };

            if let Some(idx) = next {
                e.prevent_default();
                activate_tab(&root_c, &tabs_c, idx);
                // Focus the newly activated tab.
                let _ = tabs_c[idx].focus();
            }
        });
        tablist_node
            .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
            .unwrap_or_default();
        on_keydown.forget();
    }

    dispatch_initialized(&root, "tabs");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn collect_role(container: &Element, role: &str) -> Vec<HtmlElement> {
    let selector = format!("[role='{}']", role);
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

fn current_index(tabs: &[HtmlElement]) -> usize {
    let document = web_sys::window().and_then(|w| w.document());
    let active = document.and_then(|d| d.active_element());
    if let Some(a) = active
        && let Some(idx) = tabs.iter().position(|t| {
            let t_el: &Element = t.as_ref();
            t_el == &a
        })
    {
        return idx;
    }
    // Fall back to aria-selected tab.
    tabs.iter()
        .position(|t| t.get_attribute("aria-selected").as_deref() == Some("true"))
        .unwrap_or(0)
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

fn dispatch_initialized(el: &Element, name: &str) {
    use web_sys::{CustomEvent, CustomEventInit};
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
