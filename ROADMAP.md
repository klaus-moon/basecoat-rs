# Roadmap

This document captures the rationale for what was IN vs OUT of v0.1 and what
v0.2 needs to build before the deferred components can ship.

The v0.1 release intentionally drew a line around components that need a
floating-element positioning engine or non-trivial keyboard navigation. v0.2
work is gated on building that shared infrastructure first.

---

## What v0.1 shipped, and why those were tractable

| Component | Why it was easy |
|---|---|
| Button, Badge, Alert, Card, Input, Label, Separator, Textarea | Pure markup + Tailwind classes. No JS, no positioning, no state. |
| Tooltip | Pure CSS — no positioning helper, just `[data-tooltip]` styling. |
| Dialog | Uses the native `<dialog>` element + `showModal()`. Browser handles the modal layer, focus trap, and Esc-to-close. |
| Tabs | In-flow tablist. State is local, no positioning. Keyboard nav is simple (Left/Right + Home/End). |
| Toast / Toaster | Fixed-position viewport container. The controller appends children, never anchors to a trigger. |

Common thread: nothing in v0.1 needs to compute "where does this float, relative
to its anchor, without going off-screen."

---

## What v0.2 needs first: shared infrastructure

The five deferred components (Dropdown, Popover, Select, Sidebar, Combobox) all
fail at least one v0.1 constraint. They share missing infrastructure that has to
land before any of them can ship.

### 1. Floating-element positioning engine

Required by: Dropdown, Popover, Select, Combobox.

What we need:
- Anchored placement (`top|right|bottom|left` with `start|center|end` alignment).
- Collision detection against the viewport (flip to the opposite side when there
  isn't room).
- Shift-along-axis to keep the popover on-screen when the anchor is near an edge.
- Re-position on scroll, resize, and ancestor scroll.
- Arrow element positioning (Popover only — Dropdown/Select usually skip the arrow).

Three options, in increasing order of effort:

- **Option A — CSS Anchor Positioning** (`anchor-name` / `position-anchor` /
  `position-try`). Stable in Chrome 125+, behind a flag in Safari, not yet in
  Firefox. Browser support insufficient as of 2026; would need a JS fallback,
  which defeats the simplicity argument.
- **Option B — port a subset of floating-ui to Rust/WASM.** Conceptually the
  cleanest match (the algorithm is well-documented and framework-agnostic), but
  it's real work — the JS lib is ~12KB minified. We only need
  `computePosition`, `flip`, `shift`, `offset`, `arrow`, and a
  `autoUpdate`-equivalent that listens to scroll/resize.
- **Option C — call into floating-ui from JS** via the existing
  `basecoat-controllers` module. Bundle floating-ui's `dom` build alongside the
  WASM init script. Pragmatic; trades bundle size for shipping speed.

Recommended: **C for v0.2, evaluate B for v0.3.** Shipping is more valuable than
purity right now.

### 2. Roving-tabindex / keyboard navigation utility

Required by: Dropdown menu items, Select listbox, Combobox listbox, Sidebar
nav links (optional).

What we need:
- A small controller helper that takes a list of items and routes
  ArrowUp/ArrowDown/Home/End/Type-to-search to focus the right element.
- Wrap-around and disabled-item skipping.
- Integration with the WASM hydration model — the controller attaches once on
  hydrate and survives DOM updates.

Live in `basecoat-controllers/src/controllers/keyboard.rs` (new file). All four
listbox-style components share this helper.

### 3. Click-outside / dismiss-on-escape primitive

Required by: Dropdown, Popover, Select, Combobox.

A single shared `dismiss` controller that watches:
- `click` outside the floating element (capture phase, but skip clicks on the
  trigger so re-toggling works).
- `keydown` Escape.
- `focusin` outside the floating element + trigger.

Each consumer registers a callback. Avoids each component re-implementing this.

### 4. ARIA combobox / listbox patterns

Required by: Combobox, Select.

The WAI-ARIA APG patterns (`combobox`, `listbox`, `option`, `aria-activedescendant`
vs roving tabindex) are well-defined but verbose to get right. v0.1 has none of
this scaffolding.

---

## Per-component notes

### Dropdown

- Trigger renders `aria-haspopup="menu"` + `aria-expanded`.
- Menu has `role="menu"`, items have `role="menuitem"`.
- Submenu support is a v0.3 question — punt on it for v0.2.
- Infrastructure needed: floating-positioning, roving-tabindex, dismiss.

### Popover

- Just floating content anchored to a trigger. No menu semantics.
- Arrow element is the only piece Dropdown doesn't share.
- Infrastructure needed: floating-positioning, dismiss.

### Select

- The hardest of the four floating components — needs form integration.
- Renders a hidden `<select>` so it works without JS and submits with forms.
- Visible button mirrors the hidden select's value; clicking opens a styled
  listbox; selecting an option updates both.
- Search-by-typing is table-stakes (jumps to options starting with the typed
  prefix within ~500ms).
- Infrastructure needed: floating-positioning, roving-tabindex, dismiss, listbox
  ARIA.

### Sidebar

- Structurally different from the other four — not a floating component.
- Stateful: collapsed/expanded, with state often persisted to localStorage and
  to a server cookie for SSR.
- Responsive: above breakpoint X it's in-flow, below X it's an overlay drawer.
- Infrastructure needed: a media-query controller (react to breakpoint changes),
  a persistence helper (localStorage + optional cookie), and the dismiss
  primitive for the overlay drawer mode.
- Consider whether sidebar belongs in v0.2 or splits into its own milestone —
  it's the largest single component by far.

### Combobox

- Save for last; it's the most complex.
- Input element + filtered listbox + selection state + free-form vs
  selection-only modes.
- Async option loading is a likely follow-up — design the prop surface so it
  can take both static and async option providers without a breaking change.
- Infrastructure needed: everything above.

---

## Proposed v0.2 milestone shape

Two ways to slice this — pick one before writing code:

- **Infra-first**: ship v0.2.0-alpha.0 with just the positioning + keyboard +
  dismiss primitives (no new components). Then v0.2.0 adds Dropdown +
  Popover. v0.2.1 adds Select. v0.2.2 adds Combobox. Sidebar goes to v0.3.
- **Vertical slice**: build positioning + dismiss + Dropdown in one PR as the
  "thin first slice" that proves the architecture. Then add components one at
  a time, each PR pulling in only the additional infra (roving-tabindex for
  Select, listbox ARIA for Combobox, etc.).

The vertical-slice path is faster to user-visible value; the infra-first path
gives a cleaner library API (downstream code can depend on the primitives even
before the components ship).

---

## Open questions to decide before v0.2 work starts

1. **floating-ui JS dependency, or port to Rust?** (See Option C vs B above.)
   This is the single biggest architectural call for v0.2.
2. **Sidebar in v0.2, or push to v0.3?** Sidebar's persistence + responsive
   behavior is genuinely different work from the other four.
3. **Form integration for Select.** Hidden `<select>` is the obvious choice;
   confirm before designing the prop surface.
4. **Leptos adapter parity.** Every shipped component has a Leptos wrapper.
   Confirm Dropdown/Popover/Select/Combobox can model their open-state cleanly
   with Leptos signals before locking the API.
5. **Headless vs styled defaults.** v0.1 ships strongly opinionated styling.
   Decide whether the listbox/menu items expose a `class` slot for visual
   customization or stay fully styled.

---

## Not on the v0.2 list (yet)

- Accordion, Collapsible — could land in v0.2 trivially (in-flow, no floating
  needed). Add to v0.2 if they're useful, but they weren't called out in the
  v0.1 README's feature matrix and aren't blocking anyone.
- Command (cmdk-style palette) — depends on Combobox + Dialog interaction.
  v0.3 candidate.
- DataTable — out of scope; belongs in a separate crate.

---

*This document is forward-looking. The v0.1 deferral rationale is reconstructed
from what's in vs out of the codebase — there was no design doc written at the
time of the v0.1 cut. Treat the "infrastructure needed" lists as starting points,
not commitments.*
