// Floating-UI bridge.
//
// Thin Rust wrapper over `@floating-ui/dom` (loaded as an ES module from
// `js/floating.js`). wasm-pack copies the file into
// `pkg/snippets/basecoat-controllers-<hash>/js/floating.js` at build time;
// the JS module loader resolves the relative `../vendor/...` import inside
// it correctly.
//
// The vendored `@floating-ui/dom` ESM bundle is copied to `pkg/vendor/` by
// xtask `cmd_build_wasm`; see `xtask/src/main.rs`.

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/js/floating.js")]
extern "C" {
    /// Compute a position for `floating` relative to `reference` using
    /// `@floating-ui/dom`. Resolves with the result object (`{x, y, ...}`)
    /// and synchronously mutates `floating.style` to apply the position.
    #[wasm_bindgen(js_name = compute_position)]
    pub fn compute_position(
        reference: &web_sys::Element,
        floating: &web_sys::HtmlElement,
        placement: &str,
        offset: f64,
    ) -> js_sys::Promise;

    /// Subscribe to `autoUpdate` for `reference`/`floating`. The returned
    /// function tears the subscription down — callers must invoke it on
    /// dismiss to avoid leaks.
    #[wasm_bindgen(js_name = auto_update)]
    pub fn auto_update(
        reference: &web_sys::Element,
        floating: &web_sys::HtmlElement,
        callback: &js_sys::Function,
    ) -> js_sys::Function;
}

/// Convenience wrapper: await `compute_position`, ignore the result.
///
/// The JS shim applies `left` / `top` / `position: absolute` itself, so
/// most callers do not need the returned object. Default offset is 4px.
pub async fn position(
    reference: &web_sys::Element,
    floating: &web_sys::HtmlElement,
    placement: &str,
) {
    let promise = compute_position(reference, floating, placement, 4.0);
    let _ = JsFuture::from(promise).await;
}
