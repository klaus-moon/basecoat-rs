// `window.matchMedia` wrapper.
//
// Used by Sidebar (and future responsive components) to react to viewport
// changes. Listener lifetime is the caller's responsibility — drop the
// `MediaHandle` to detach.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{MediaQueryList, MediaQueryListEvent};

/// Synchronously evaluate a media query and return its current match state.
/// Returns `false` if no window is available (SSR or otherwise).
pub fn matches(query: &str) -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(Some(mql)) = window.match_media(query) else {
        return false;
    };
    mql.matches()
}

pub struct MediaHandle {
    mql: MediaQueryList,
    listener: Option<Closure<dyn Fn(MediaQueryListEvent)>>,
}

impl Drop for MediaHandle {
    fn drop(&mut self) {
        let Some(closure) = self.listener.take() else {
            return;
        };
        let target: &web_sys::EventTarget = self.mql.as_ref();
        let _ = target
            .remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
    }
}

/// Invoke `callback` with the new match state every time `query` changes.
/// The callback is NOT invoked synchronously with the current value; call
/// [`matches`] separately if you need the initial state.
pub fn on_change(query: &str, callback: Box<dyn Fn(bool)>) -> MediaHandle {
    let window = web_sys::window().expect("on_change requires a window");
    let mql = window
        .match_media(query)
        .ok()
        .flatten()
        .expect("matchMedia returned no MediaQueryList");

    let closure = Closure::<dyn Fn(MediaQueryListEvent)>::new(move |e: MediaQueryListEvent| {
        callback(e.matches());
    });
    let target: &web_sys::EventTarget = mql.as_ref();
    let _ = target.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());

    MediaHandle {
        mql,
        listener: Some(closure),
    }
}
