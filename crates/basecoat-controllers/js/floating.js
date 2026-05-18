// Floating-UI bridge — loaded by wasm-bindgen via `module = "/js/floating.js"`.
//
// wasm-pack copies this file into `pkg/snippets/<crate>-<hash>/js/floating.js`.
// The relative `../vendor/floating-ui.dom.esm.js` import resolves to
// `pkg/vendor/floating-ui.dom.esm.js`, which xtask `cmd_build_wasm` copies
// out of `node_modules/@floating-ui/dom/dist/` after every build.

export async function compute_position(reference, floating, placement, offsetPx) {
    const { computePosition, flip, shift, offset } = await import(
        '../vendor/floating-ui.dom.esm.js'
    );
    const result = await computePosition(reference, floating, {
        placement,
        middleware: [offset(offsetPx), flip(), shift({ padding: 8 })],
    });
    Object.assign(floating.style, {
        position: 'absolute',
        left: `${result.x}px`,
        top: `${result.y}px`,
    });
    return result;
}

export function auto_update(reference, floating, callback) {
    // Dynamic import inside autoUpdate is okay; auto_update is called
    // infrequently (once per popover open).
    let cleanup = () => {};
    let disposed = false;
    (async () => {
        const { autoUpdate } = await import('../vendor/floating-ui.dom.esm.js');
        if (disposed) {
            return;
        }
        cleanup = autoUpdate(reference, floating, callback);
    })();
    return () => {
        disposed = true;
        cleanup();
    };
}
