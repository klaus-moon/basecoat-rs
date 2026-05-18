// Roving tabindex helper.
//
// Implements the WAI-ARIA "roving tabindex" pattern: a focusable collection
// where only one item is in the tab sequence (`tabindex="0"`) at any time;
// arrow keys move focus among the rest (`tabindex="-1"`).
//
// Used by tabs (horizontal), and intended to be reused by menus, listboxes,
// segmented controls, and any other widget that wants single-stop tabbing
// with arrow-key navigation.
//
// Listener lifetime: `RovingHandle` owns its `Closure`s. Dropping the handle
// detaches every listener — diverges from dialog.rs's `Closure::forget()`
// pattern because callers (menus, combobox listboxes) need to swap or destroy
// the item set dynamically.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, KeyboardEvent};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
    Both,
}

pub struct RovingOpts {
    pub orientation: Orientation,
    pub wrap: bool,
    pub type_ahead: bool,
}

impl Default for RovingOpts {
    fn default() -> Self {
        Self {
            orientation: Orientation::Both,
            wrap: true,
            type_ahead: false,
        }
    }
}

/// Owns the `Closure` keeping the keydown listener alive. Drop to detach.
pub struct RovingHandle {
    container: Element,
    keydown: Option<Closure<dyn Fn(KeyboardEvent)>>,
}

impl Drop for RovingHandle {
    fn drop(&mut self) {
        let Some(closure) = self.keydown.take() else {
            return;
        };
        let target: &web_sys::EventTarget = self.container.as_ref();
        let _ = target.remove_event_listener_with_callback(
            "keydown",
            closure.as_ref().unchecked_ref(),
        );
    }
}

/// State shared between the keydown listener and the type-ahead buffer.
struct State {
    items: Vec<HtmlElement>,
    opts: RovingOpts,
    type_buffer: String,
    last_type_at: f64,
}

const TYPE_AHEAD_TIMEOUT_MS: f64 = 500.0;

/// Wire roving tabindex on `container` for the supplied `items`.
///
/// Updates `tabindex` on every item immediately: the currently focused or
/// `aria-selected="true"` item gets `0`, all others `-1`. Returns a handle
/// whose `Drop` removes the keydown listener.
pub fn attach(container: &Element, items: Vec<HtmlElement>, opts: RovingOpts) -> RovingHandle {
    // Initial tabindex sync: prefer the aria-selected item, else first
    // enabled item, else the first item.
    let initial = items
        .iter()
        .position(|el| el.get_attribute("aria-selected").as_deref() == Some("true"))
        .or_else(|| items.iter().position(is_enabled))
        .unwrap_or(0);
    sync_tabindex(&items, initial);

    let state = Rc::new(RefCell::new(State {
        items,
        opts,
        type_buffer: String::new(),
        last_type_at: 0.0,
    }));

    let state_for_listener = state.clone();
    let closure = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        handle_keydown(&state_for_listener, e);
    });
    let target: &web_sys::EventTarget = container.as_ref();
    let _ = target
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());

    RovingHandle {
        container: container.clone(),
        keydown: Some(closure),
    }
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

fn handle_keydown(state: &Rc<RefCell<State>>, e: KeyboardEvent) {
    let key = e.key();
    let mut state_mut = state.borrow_mut();

    if state_mut.items.is_empty() {
        return;
    }

    let horizontal = matches!(
        state_mut.opts.orientation,
        Orientation::Horizontal | Orientation::Both
    );
    let vertical = matches!(
        state_mut.opts.orientation,
        Orientation::Vertical | Orientation::Both
    );

    let current = current_index(&state_mut.items);
    let len = state_mut.items.len();

    let direction = match key.as_str() {
        "ArrowRight" if horizontal => Some(Direction::Next),
        "ArrowLeft" if horizontal => Some(Direction::Prev),
        "ArrowDown" if vertical => Some(Direction::Next),
        "ArrowUp" if vertical => Some(Direction::Prev),
        "Home" => Some(Direction::First),
        "End" => Some(Direction::Last),
        _ => None,
    };

    if let Some(dir) = direction {
        e.prevent_default();
        let next = find_next(&state_mut.items, current, dir, state_mut.opts.wrap, len);
        if let Some(idx) = next {
            sync_tabindex(&state_mut.items, idx);
            let _ = state_mut.items[idx].focus();
        }
        return;
    }

    // Type-ahead: single printable character key.
    if state_mut.opts.type_ahead && key.chars().count() == 1 {
        let ch = key.chars().next().unwrap();
        if !ch.is_control() {
            let now = js_sys::Date::now();
            if now - state_mut.last_type_at > TYPE_AHEAD_TIMEOUT_MS {
                state_mut.type_buffer.clear();
            }
            state_mut.type_buffer.push(ch.to_ascii_lowercase());
            state_mut.last_type_at = now;

            let buffer = state_mut.type_buffer.clone();
            if let Some(idx) = find_by_prefix(&state_mut.items, current, &buffer) {
                e.prevent_default();
                sync_tabindex(&state_mut.items, idx);
                let _ = state_mut.items[idx].focus();
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Next,
    Prev,
    First,
    Last,
}

fn find_next(
    items: &[HtmlElement],
    current: usize,
    direction: Direction,
    wrap: bool,
    len: usize,
) -> Option<usize> {
    match direction {
        Direction::First => (0..len).find(|&i| is_enabled(&items[i])),
        Direction::Last => (0..len).rev().find(|&i| is_enabled(&items[i])),
        Direction::Next => step_forward(items, current, wrap, len),
        Direction::Prev => step_backward(items, current, wrap, len),
    }
}

fn step_forward(items: &[HtmlElement], current: usize, wrap: bool, len: usize) -> Option<usize> {
    let mut idx = current;
    for _ in 0..len {
        idx = if idx + 1 >= len {
            if wrap { 0 } else { return None }
        } else {
            idx + 1
        };
        if is_enabled(&items[idx]) {
            return Some(idx);
        }
    }
    None
}

fn step_backward(items: &[HtmlElement], current: usize, wrap: bool, len: usize) -> Option<usize> {
    let mut idx = current;
    for _ in 0..len {
        idx = if idx == 0 {
            if wrap { len - 1 } else { return None }
        } else {
            idx - 1
        };
        if is_enabled(&items[idx]) {
            return Some(idx);
        }
    }
    None
}

fn find_by_prefix(items: &[HtmlElement], current: usize, prefix: &str) -> Option<usize> {
    if prefix.is_empty() {
        return None;
    }
    let len = items.len();
    // Search starting just after current, wrap around.
    for offset in 1..=len {
        let idx = (current + offset) % len;
        if !is_enabled(&items[idx]) {
            continue;
        }
        let text = items[idx].text_content().unwrap_or_default();
        if text.trim().to_ascii_lowercase().starts_with(prefix) {
            return Some(idx);
        }
    }
    None
}

fn current_index(items: &[HtmlElement]) -> usize {
    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());
    if let Some(a) = active
        && let Some(idx) = items.iter().position(|t| {
            let t_el: &Element = t.as_ref();
            t_el == &a
        })
    {
        return idx;
    }
    items
        .iter()
        .position(|t| t.get_attribute("tabindex").as_deref() == Some("0"))
        .unwrap_or(0)
}

pub fn is_enabled(el: &HtmlElement) -> bool {
    if el.has_attribute("disabled") {
        return false;
    }
    if el.get_attribute("aria-disabled").as_deref() == Some("true") {
        return false;
    }
    true
}

fn sync_tabindex(items: &[HtmlElement], active: usize) {
    for (i, item) in items.iter().enumerate() {
        let value = if i == active { "0" } else { "-1" };
        let _ = item.set_attribute("tabindex", value);
    }
}
