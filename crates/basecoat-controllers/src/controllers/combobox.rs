// Combobox controller
//
// Wires an input + listbox pair following the WAI-ARIA combobox pattern with
// the `aria-activedescendant` strategy (focus stays on the input; the
// highlighted option is announced via aria-activedescendant). This diverges
// intentionally from the shared `super::keyboard` helper, which implements
// roving tabindex — moving focus into the listbox would defeat the input's
// ability to receive keystrokes for filtering.
//
// Behaviors:
//   - Focus or click on the input → open listbox, position via floating-UI.
//   - Typing → filter options by case-insensitive substring on label, hide
//     non-matches, point aria-activedescendant at the first visible match.
//   - ArrowDown / ArrowUp → move aria-activedescendant among visible options
//     (wrap-around).
//   - PageDown / PageUp → jump 5 visible options.
//   - Home / End → jump to first / last visible option.
//   - Enter → select current aria-activedescendant, close listbox, dispatch a
//     synthetic `change` event on the input.
//   - Escape, outside click, focus-out → close listbox (via shared dismiss).
//   - Option click → select that option.
//
// Listener lifetime: combobox state (filter, dismiss handle, autoUpdate
// teardown) is kept in a `ComboboxState` inside an Rc<RefCell<_>>. Closures
// attached to the input/listbox themselves use `Closure::forget()` for the
// same reason as dialog.rs (page-lifetime listeners).

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    CustomEvent, CustomEventInit, Element, FocusEvent, HtmlButtonElement, HtmlElement,
    HtmlInputElement, KeyboardEvent, MouseEvent,
};

use super::dismiss::{self, DismissHandle};
use super::floating;
use super::util::dispatch_initialized;

const PAGE_JUMP: usize = 5;

struct OptionEntry {
    element: HtmlButtonElement,
    id: String,
    label_lower: String,
    visible: bool,
}

struct ComboboxState {
    root: Element,
    input: HtmlInputElement,
    listbox: HtmlElement,
    options: Vec<OptionEntry>,
    active: Option<usize>,
    open: bool,
    dismiss: Option<DismissHandle>,
}

pub fn attach(root: Element) {
    let input_el = match root.query_selector("[data-combobox-input]") {
        Ok(Some(el)) => el,
        _ => {
            web_sys::console::warn_1(
                &"[basecoat:combobox] no [data-combobox-input] found".into(),
            );
            return;
        }
    };
    let input = match input_el.dyn_into::<HtmlInputElement>() {
        Ok(i) => i,
        Err(_) => return,
    };

    let listbox_el = match root.query_selector("[data-combobox-listbox]") {
        Ok(Some(el)) => el,
        _ => {
            web_sys::console::warn_1(
                &"[basecoat:combobox] no [data-combobox-listbox] found".into(),
            );
            return;
        }
    };
    let listbox = match listbox_el.dyn_into::<HtmlElement>() {
        Ok(l) => l,
        Err(_) => return,
    };

    let options = collect_options(&listbox);

    let state = Rc::new(RefCell::new(ComboboxState {
        root: root.clone(),
        input: input.clone(),
        listbox: listbox.clone(),
        options,
        active: None,
        open: false,
        dismiss: None,
    }));

    // ---- Focus on input: open listbox ---------------------------------------
    {
        let state_c = state.clone();
        let on_focus = Closure::<dyn Fn(FocusEvent)>::new(move |_e: FocusEvent| {
            open_listbox(&state_c);
        });
        let target: &web_sys::EventTarget = input.as_ref();
        let _ = target
            .add_event_listener_with_callback("focus", on_focus.as_ref().unchecked_ref());
        on_focus.forget();
    }

    // ---- Click on input: open (in case it was closed but still focused) ----
    {
        let state_c = state.clone();
        let on_click = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
            open_listbox(&state_c);
        });
        let target: &web_sys::EventTarget = input.as_ref();
        let _ = target
            .add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
        on_click.forget();
    }

    // ---- Input event: filter ------------------------------------------------
    {
        let state_c = state.clone();
        let on_input = Closure::<dyn Fn(web_sys::Event)>::new(move |_e: web_sys::Event| {
            filter_options(&state_c);
            open_listbox(&state_c);
        });
        let target: &web_sys::EventTarget = input.as_ref();
        let _ = target
            .add_event_listener_with_callback("input", on_input.as_ref().unchecked_ref());
        on_input.forget();
    }

    // ---- Keydown on input: navigation + selection ---------------------------
    {
        let state_c = state.clone();
        let on_keydown = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            handle_keydown(&state_c, &e);
        });
        let target: &web_sys::EventTarget = input.as_ref();
        let _ = target
            .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref());
        on_keydown.forget();
    }

    // ---- Click on options: select -------------------------------------------
    {
        let snapshot = state.borrow();
        for (idx, entry) in snapshot.options.iter().enumerate() {
            let state_c = state.clone();
            let on_click = Closure::<dyn Fn(MouseEvent)>::new(move |_e: MouseEvent| {
                select_option(&state_c, idx);
            });
            let target: &web_sys::EventTarget = entry.element.as_ref();
            let _ = target
                .add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
            on_click.forget();
        }
    }

    set_state_attr(&root, "closed");
    dispatch_initialized(&root, "combobox");
}

// ---------------------------------------------------------------------------
// Behaviors
// ---------------------------------------------------------------------------

fn open_listbox(state: &Rc<RefCell<ComboboxState>>) {
    if state.borrow().open {
        return;
    }

    let (input, listbox, root) = {
        let s = state.borrow();
        (s.input.clone(), s.listbox.clone(), s.root.clone())
    };

    let _ = listbox.remove_attribute("hidden");
    let _ = input.set_attribute("aria-expanded", "true");
    set_state_attr(&root, "open");

    // Position via floating-UI (async).
    {
        let input_ref: Element = input.clone().into();
        let floating_el = listbox.clone();
        spawn_local(async move {
            floating::position(&input_ref, &floating_el, "bottom-start").await;
        });
    }

    // Wire dismiss listeners (Escape, outside click, focus-out).
    let dismiss_handle = {
        let state_c = state.clone();
        dismiss::attach(
            listbox.clone(),
            input.clone().unchecked_into::<HtmlElement>(),
            Box::new(move || {
                close_listbox(&state_c);
            }),
        )
    };

    {
        let mut s = state.borrow_mut();
        s.open = true;
        s.dismiss = Some(dismiss_handle);
        if s.active.is_none() {
            s.active = first_visible_index(&s.options);
        }
    }
    refresh_active_descendant(state);
}

fn close_listbox(state: &Rc<RefCell<ComboboxState>>) {
    if !state.borrow().open {
        return;
    }
    let (input, listbox, root) = {
        let s = state.borrow();
        (s.input.clone(), s.listbox.clone(), s.root.clone())
    };

    let _ = listbox.set_attribute("hidden", "");
    let _ = input.set_attribute("aria-expanded", "false");
    let _ = input.remove_attribute("aria-activedescendant");
    set_state_attr(&root, "closed");

    let mut s = state.borrow_mut();
    s.open = false;
    s.dismiss.take(); // drop handle → detaches listeners
    for entry in &mut s.options {
        let _ = entry.element.remove_attribute("data-active");
    }
}

fn filter_options(state: &Rc<RefCell<ComboboxState>>) {
    let query = state.borrow().input.value().trim().to_lowercase();
    let mut s = state.borrow_mut();
    for entry in &mut s.options {
        let visible = query.is_empty() || entry.label_lower.contains(&query);
        entry.visible = visible;
        if visible {
            let _ = entry.element.remove_attribute("hidden");
        } else {
            let _ = entry.element.set_attribute("hidden", "");
        }
    }
    s.active = first_visible_index(&s.options);
    drop(s);
    refresh_active_descendant(state);
}

fn handle_keydown(state: &Rc<RefCell<ComboboxState>>, e: &KeyboardEvent) {
    match e.key().as_str() {
        "ArrowDown" => {
            e.prevent_default();
            if !state.borrow().open {
                open_listbox(state);
                return;
            }
            move_active(state, Movement::Next(1));
        }
        "ArrowUp" => {
            e.prevent_default();
            if !state.borrow().open {
                open_listbox(state);
                return;
            }
            move_active(state, Movement::Prev(1));
        }
        "PageDown" => {
            if !state.borrow().open {
                return;
            }
            e.prevent_default();
            move_active(state, Movement::Next(PAGE_JUMP));
        }
        "PageUp" => {
            if !state.borrow().open {
                return;
            }
            e.prevent_default();
            move_active(state, Movement::Prev(PAGE_JUMP));
        }
        "Home" => {
            if !state.borrow().open {
                return;
            }
            e.prevent_default();
            move_active(state, Movement::First);
        }
        "End" => {
            if !state.borrow().open {
                return;
            }
            e.prevent_default();
            move_active(state, Movement::Last);
        }
        "Enter" => {
            let active = state.borrow().active;
            if let Some(idx) = active {
                e.prevent_default();
                select_option(state, idx);
            }
        }
        "Escape" => {
            // dismiss::attach also handles Escape, but matching it here means
            // we never need to bubble up to the document if the listbox is
            // open and the input has focus.
            if state.borrow().open {
                e.prevent_default();
                close_listbox(state);
            }
        }
        _ => {}
    }
}

fn select_option(state: &Rc<RefCell<ComboboxState>>, idx: usize) {
    let value = {
        let s = state.borrow();
        let Some(entry) = s.options.get(idx) else {
            return;
        };
        entry
            .element
            .get_attribute("data-value")
            .unwrap_or_else(|| entry.element.text_content().unwrap_or_default())
    };

    {
        let s = state.borrow();
        s.input.set_value(&value);
    }
    dispatch_change(&state.borrow().input);

    // Re-filter so the listbox reflects the now-selected value, then close.
    filter_options(state);
    close_listbox(state);
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Movement {
    Next(usize),
    Prev(usize),
    First,
    Last,
}

fn move_active(state: &Rc<RefCell<ComboboxState>>, movement: Movement) {
    let next = {
        let s = state.borrow();
        let visible: Vec<usize> = s
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, e)| e.visible.then_some(i))
            .collect();
        if visible.is_empty() {
            None
        } else {
            let current_pos = s
                .active
                .and_then(|active| visible.iter().position(|&v| v == active));
            let next_pos = compute_next(current_pos, visible.len(), movement);
            Some(visible[next_pos])
        }
    };

    state.borrow_mut().active = next;
    refresh_active_descendant(state);
}

fn compute_next(current: Option<usize>, len: usize, movement: Movement) -> usize {
    match movement {
        Movement::First => 0,
        Movement::Last => len - 1,
        Movement::Next(step) => {
            let base = current.map(|c| c + step).unwrap_or(0);
            base % len
        }
        Movement::Prev(step) => {
            let cur = current.unwrap_or(0);
            // Wrap-around subtraction without going negative.
            let len_i = len as isize;
            let mut next = cur as isize - step as isize;
            while next < 0 {
                next += len_i;
            }
            (next as usize) % len
        }
    }
}

fn refresh_active_descendant(state: &Rc<RefCell<ComboboxState>>) {
    let s = state.borrow();
    for entry in &s.options {
        let _ = entry.element.remove_attribute("data-active");
    }
    let Some(idx) = s.active else {
        let _ = s.input.remove_attribute("aria-activedescendant");
        return;
    };
    let Some(entry) = s.options.get(idx) else {
        let _ = s.input.remove_attribute("aria-activedescendant");
        return;
    };
    let _ = s.input.set_attribute("aria-activedescendant", &entry.id);
    let _ = entry.element.set_attribute("data-active", "true");
    entry.element.scroll_into_view_with_bool(false);
}

fn first_visible_index(options: &[OptionEntry]) -> Option<usize> {
    options.iter().position(|e| e.visible)
}

fn collect_options(listbox: &HtmlElement) -> Vec<OptionEntry> {
    let node_list = match listbox.query_selector_all("[role='option']") {
        Ok(nl) => nl,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for i in 0..node_list.length() {
        let Some(node) = node_list.item(i) else {
            continue;
        };
        let Ok(button) = node.dyn_into::<HtmlButtonElement>() else {
            continue;
        };
        let id = button.id();
        let label = button.text_content().unwrap_or_default();
        out.push(OptionEntry {
            element: button,
            id,
            label_lower: label.trim().to_lowercase(),
            visible: true,
        });
    }
    out
}

fn set_state_attr(el: &Element, state: &str) {
    let _ = el.set_attribute("data-state", state);
}

fn dispatch_change(input: &HtmlInputElement) {
    let init = CustomEventInit::new();
    init.set_bubbles(true);
    let Ok(ev) = CustomEvent::new_with_event_init_dict("change", &init) else {
        return;
    };
    let target: &web_sys::EventTarget = input.as_ref();
    let _ = target.dispatch_event(&ev);
}
