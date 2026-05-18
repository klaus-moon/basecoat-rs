# Roadmap

Per-version scope, decisions made, and what's next.

---

## v0.1 — what shipped (2026-05-18)

13 components, all either pure-markup or built around primitives that don't
need positioning math. Common thread: nothing in v0.1 needs to compute "where
does this float, relative to its anchor, without going off-screen."

| Component | Why it was tractable |
|---|---|
| Button, Badge, Alert, Card, Input, Label, Separator, Textarea | Pure markup + Tailwind classes. |
| Tooltip | Pure CSS — `[data-tooltip]` styling, no JS. |
| Dialog | Native `<dialog>` element + `showModal()`; browser handles modal layer, focus trap, Esc-to-close. |
| Tabs | In-flow tablist; local state; simple keyboard nav (Left/Right + Home/End). |
| Toast / Toaster | Fixed-position viewport container; controller appends children, never anchors. |

---

## v0.2 — what shipped (2026-05-18)

Five components plus the shared infrastructure they all needed.

### Components

| Component | What's new |
|---|---|
| Dropdown | Anchored menu on `<details>`; floating-positioned; roving-tabindex with type-ahead. |
| Popover | Anchored content panel; twelve placement variants (`PopoverPlacement`); dismiss primitive. |
| Select | Hidden native `<select>` for form integration + styled trigger + floating listbox. |
| Sidebar | Responsive (in-flow ≥768px, overlay drawer below); `localStorage` persistence per id. |
| Combobox | Input + filtered listbox; `aria-activedescendant` keyboard nav. |

### Shared controller infrastructure

| Module | What it does |
|---|---|
| `controllers/keyboard.rs` | `RovingTabindex` — Arrow/Home/End/type-ahead, detachable. |
| `controllers/dismiss.rs` | Click-outside (capture phase) + Escape + focus-out, detachable. |
| `controllers/floating.rs` | Rust → JS bridge over `@floating-ui/dom`. |
| `controllers/media.rs` | `matchMedia` wrapper with detachable listener. |
| `controllers/util.rs` | `dispatch_initialized` extracted from dialog/tabs/toast. |

### CSS distribution

`basecoat-css` is now its own crate (`basecoat::css::SOURCE`) and an npm
package (`basecoat-rs-css`). Consumers no longer have to vendor
`style/basecoat.css`.

### Decisions made (v0.1 open questions resolved)

1. **Floating positioning approach** — Option C: bundle `@floating-ui/dom`
   as a JS shim. The vendored file ships at
   `pkg/vendor/floating-ui.dom.esm.js`; the WASM module loads it through a
   small ES-module shim. **A Rust port (Option B) is deferred to v0.3** —
   the bundle-size gap (~12KB minified) is not yet justified by the
   ergonomic cost of crossing the JS boundary.
2. **Select form integration** — hidden `<select>`. Same as the upstream
   basecoat reference; works without JS; submits with forms.
3. **Sidebar in v0.2** — yes. Sidebar shipped in the same milestone as the
   floating components.
4. **Leptos adapter parity** — all five components have Leptos wrappers
   with compound sub-components matching the upstream HTML structure.
5. **Styling slot** — class slots are exposed via the existing `class`
   prop on every component. Listbox/menu items inherit the parent's slot.

---

## v0.3 — candidates (not committed)

These are next-most-likely additions, in no particular order:

- **Accordion / Collapsible** — in-flow disclosure components. Trivial to
  build once the dismiss/keyboard primitives are in place.
- **Command palette** (cmdk-style) — Combobox + Dialog interaction. The
  primitives exist; mostly composition work.
- **Floating positioning Rust port** — replace `@floating-ui/dom` with a
  Rust crate implementing `computePosition` + `flip` + `shift` + `offset`
  + `autoUpdate`. Trades JS-bridge crossings for a ~12KB Rust dep.
- **Accordion / Collapsible** for nav structures inside Sidebar.
- **DataTable** — out of scope for the core component crate; would belong
  in a separate add-on crate.

---

## Versioning policy

All workspace crates bump together (`basecoat`, `basecoat-core`,
`basecoat-components`, `basecoat-controllers`, `basecoat-leptos`,
`basecoat-css`, `basecoat-core-macros`, `basecoat-macros`,
`basecoat-macros-rt`). The umbrella `basecoat` version is the project's
public version number.

`data-basecoat-version="0.X"` on hydrated elements tracks the markup
contract version — the controller treats `"0.1"` and `"0.2"` as
forward-compatible (both hydrate cleanly with the current WASM bundle).
