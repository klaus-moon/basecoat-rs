# basecoat-rs `rsx!` Macro — Syntax Specification

Version: 0.1  
Status: **Locked** (changes require trybuild snapshot updates)

---

## Table of Contents

1. [Overview](#overview)
2. [Tags: Components vs HTML Elements](#tags-components-vs-html-elements)
3. [Attributes](#attributes)
4. [Children](#children)
5. [Fragments](#fragments)
6. [Self-closing Tags](#self-closing-tags)
7. [Control Flow](#control-flow)
8. [HTML Element Rendering](#html-element-rendering)
9. [Void Elements](#void-elements)
10. [Escaping Rules](#escaping-rules)
11. [Output Type](#output-type)
12. [Worked Examples](#worked-examples)
13. [Component Name Mapping](#component-name-mapping)
14. [Known Props vs Passthrough Attrs](#known-props-vs-passthrough-attrs)

---

## Overview

`rsx! { ... }` is a proc-macro that accepts JSX-like syntax and expands to a
`::basecoat_core::Markup` value (a newtype over `Cow<'static, str>` containing
pre-rendered HTML). It is **not** a reactivity primitive — it is a string
emitter. No signals, no closures, no event handlers.

The macro is built on top of [`rstml`](https://docs.rs/rstml) for tokenization.
Attribute syntax and component-vs-element disambiguation are defined by this
document, not by rstml.

---

## Tags: Components vs HTML Elements

**PascalCase tags** (first character is uppercase ASCII) are **components**.

```rust
<Button />
<DialogContent />
<TabsList />
```

Components map to function calls in `::basecoat_components`. The tag name is
converted to snake_case: every capital letter after the first is replaced with
`_` + lowercase.

| Tag | Function call |
|-----|--------------|
| `<Button>` | `::basecoat_components::button(...)` |
| `<DialogContent>` | `::basecoat_components::dialog_content(...)` |
| `<TabsList>` | `::basecoat_components::tabs_list(...)` |
| `<Alert>` | `::basecoat_components::alert(...)` |

**Lowercase / hyphenated tags** are **raw HTML elements** and are emitted as
HTML string concatenation without any function calls.

```rust
<div class="foo">...</div>
<span id="bar" />
<ul>...</ul>
```

The distinction is based solely on the first character of the **local** name
(the first segment before any punctuation). `<my-element>` is treated as an
HTML element because it starts with lowercase `m`.

---

## Attributes

Two syntactic forms are accepted for both components and HTML elements:

### Literal form

```rust
attr="value"
```

The value is a `&'static str`. The attribute name can contain hyphens, colons,
or other punctuation that rstml supports (e.g. `data-foo`, `aria-label`,
`on:click` — though `on:click` has no special semantics in basecoat-rs beyond
being stored in the `AttrMap`).

### Expression form

```rust
attr={expr}
```

`expr` is any valid Rust expression. For **HTML elements**, the expression must
evaluate to something that implements `std::fmt::Display` (the macro wraps it
in a `format!` call and HTML-escapes the result). For **components**, the
expression value is passed to the builder setter; the type is inferred by the
setter's signature.

### Boolean (value-less) attributes

```rust
<input disabled />
```

A bare attribute name with no `=` value is emitted as the attribute name with
an empty string value for HTML elements, and as `.attr("disabled", "")` for
components. This matches HTML5 boolean attribute semantics.

### Attribute routing for components

For a component tag `<Button class="foo" variant="primary" data-foo="bar">`:

- The `children` slot is populated from the tag body (see [Children](#children)).
- Every attribute is routed through the builder's typed setter when the key
  matches a known field, or through `.attr(key, value)` for everything else.
  The builder's `.attr` method calls `AttrMap::push` internally. The macro
  emits `.attr(key, value)` for **every** attribute unconditionally — the
  builder implements typed setters for known names (like `variant`, `class`,
  `size`) and `.attr` for unknown names. Since the generated builder from
  `#[derive(BasecoatProps)]` does not have an `.attr` method, the macro emits
  direct field setters for the known set of prop field names and routes
  everything else into the `AttrMap` via `.attrs(AttrMap::from_iter([...]))`.

  **Concretely:** the macro calls the builder's named setter (e.g. `.class(...)`)
  for each attribute key that matches a non-`attrs`/non-`children` field on the
  props struct, and accumulates all remaining attributes into an `AttrMap`
  passed via `.attrs(...)`.

  Because the macro does not import `basecoat-core` at expansion time (to avoid
  slowing the compile), it resolves known field names by a **static table** of
  well-known prop fields defined inside the macro crate. See
  [Known Props vs Passthrough Attrs](#known-props-vs-passthrough-attrs).

---

## Children

Content between an open and close tag — text nodes, `{expr}` blocks, and
nested tags — becomes the **`children`** argument.

```rust
<Button>"Click me"</Button>
<div>{some_var}</div>
<ul>
    { items.iter().map(|i| rsx!{ <li>{i}</li> }).collect::<String>() }
</ul>
```

Multiple children are concatenated in order. The resulting string is wrapped in
`::basecoat_core::Children::from(String)` and passed to the builder's
`.children(...)` setter.

For HTML elements, children are concatenated into the inner HTML string.

### Text node children

Quoted string literals (`"..."`) inside a tag are treated as text nodes. They
are **HTML-escaped at macro expansion time** (the macro statically escapes them
since the content is known at compile time).

### Expression block children

`{expr}` blocks evaluate at runtime. The result is passed to
`::basecoat_macros_rt::escape_text(&format!("{}", value))` which HTML-escapes
it. To emit raw (unescaped) HTML, wrap in `::basecoat_core::Markup::from(...)`:

```rust
<div>{ Markup::from_static("<b>bold</b>") }</div>
```

When the macro detects that an expression is of type `Markup` via a wrapping
call that the user writes, it still escapes it — the user must use the
`Markup::from` escape-hatch explicitly in the expression, which the runtime
will detect via a trait. In practice, for v0.1 **all `{expr}` blocks are
escaped** unless the expression itself produces a `Markup` (detected at
**runtime** by the `display_or_raw` helper in `basecoat-macros-rt`).

---

## Fragments

`<>...</>` groups multiple sibling nodes without a wrapper element. The
children are concatenated and the fragment tags emit no HTML.

```rust
rsx! {
    <>
        <li>"First"</li>
        <li>"Second"</li>
    </>
}
// Expands to: "<li>First</li><li>Second</li>"
```

Fragments may be used at the top level or nested inside other tags.

---

## Self-closing Tags

`<Tag/>` is equivalent to `<Tag></Tag>` with no children. Both forms are
accepted everywhere.

```rust
<Button variant="primary" />
// same as
<Button variant="primary"></Button>
```

---

## Control Flow

**There are no control-flow tags** (`<If>`, `<For>`, `<Show>`, etc.).

Use plain Rust inside `{expr}` blocks:

```rust
// Conditional rendering:
rsx! {
    <div>
        { if is_active { rsx!{ <span>"Active"</span> } } else { rsx!{ <span>"Inactive"</span> } } }
    </div>
}

// List rendering:
rsx! {
    <ul>
        { items.iter().map(|item| rsx!{ <li>{item}</li> }).collect::<String>() }
    </ul>
}
```

`rsx!` returns `::basecoat_core::Markup`, which implements `Display`. When used
inside an outer `rsx!` expression block `{...}`, it is converted to a string
through `Display`.

---

## HTML Element Rendering

A lowercase/hyphenated tag is rendered as an HTML string:

```
<div class="foo" id="bar">{children}</div>
```

expands to (pseudocode):

```rust
{
    let mut __s = String::new();
    __s.push_str("<div");
    __s.push_str(" class=\"foo\"");
    __s.push_str(" id=\"bar\"");
    __s.push('>');
    // children appended here
    __s.push_str("</div>");
    ::basecoat_core::Markup::from(__s)
}
```

Attribute values that are string literals are HTML-attribute-escaped at
**compile time** (they are static strings). Expression values are escaped at
**runtime** via `::basecoat_macros_rt::escape_attr`.

---

## Void Elements

The following elements are **void**: they cannot have children and are emitted
as `<tag .../>` (self-closing in XHTML style) regardless of whether the user
wrote `<br>` or `<br/>`:

```
area  base  br  col  embed  hr  img  input  link  meta  param  source  track  wbr
```

If a void element has children, the macro emits a **compile error**:

```
error: void element `<br>` cannot have children
```

---

## Escaping Rules

| Location | Input | Escaping |
|----------|-------|----------|
| Text node literal `"..."` | `&'static str` | HTML-escaped at compile time (static) |
| Expression block `{expr}` as child | `impl Display` | HTML-escaped at runtime via `escape_text` |
| Expression block `{expr}` as child (Markup) | `Markup` / `String` cast from Markup | Raw (caller's responsibility) |
| Attribute value literal `attr="..."` | `&'static str` | HTML-attr-escaped at compile time |
| Attribute value expr `attr={expr}` | `impl Display` | HTML-attr-escaped at runtime via `escape_attr` |

The five HTML characters escaped: `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;`,
`"` → `&quot;`, `'` → `&#39;`.

Runtime helpers live in `::basecoat_macros_rt` (the `basecoat-macros-rt` crate,
which must be in scope in the user's dependency tree — it is re-exported by the
umbrella `basecoat` crate).

---

## Output Type

`rsx! { ... }` always evaluates to `::basecoat_core::Markup`.

When nested inside an outer `rsx!` expression block `{ rsx!{...} }`, the inner
`Markup` is converted via its `Display` impl (which emits the raw inner HTML
string without further escaping — `Markup` carries the invariant that it is
already-escaped HTML).

---

## Worked Examples

### 1. Simple button

```rust
rsx! { <Button /> }
// Expands to:
{
    let mut __attrs = ::basecoat_core::AttrMap::new();
    ::basecoat_components::button(
        ::basecoat_core::ButtonProps::builder()
            .attrs(__attrs)
            .children(::basecoat_core::Children::from(String::new()))
            .build()
    )
}
```

### 2. Button with known prop and custom data attribute

```rust
rsx! { <Button variant="primary" data-foo="bar">"Click me"</Button> }
// Expands to:
{
    let mut __attrs = ::basecoat_core::AttrMap::new();
    __attrs.push("data-foo", "bar");
    ::basecoat_components::button(
        ::basecoat_core::ButtonProps::builder()
            .variant("primary")
            .attrs(__attrs)
            .children(::basecoat_core::Children::from({
                let mut __s = ::std::string::String::new();
                __s.push_str("Click me"); // pre-escaped literal
                __s
            }))
            .build()
    )
}
```

### 3. Nested dialog

```rust
rsx! {
    <Dialog id="my-dialog" title="Hello">
        <p>"Body content"</p>
    </Dialog>
}
```

### 4. Fragment with map (list rendering)

```rust
let items = vec!["Foo", "Bar"];
rsx! {
    <>
        { items.iter().map(|i| rsx!{ <li>{i}</li> }).collect::<String>() }
    </>
}
```

### 5. Raw `<ul>` with expression children

```rust
rsx! {
    <ul class="list">
        { items.iter().map(|i| rsx!{ <li>{i}</li> }).collect::<String>() }
    </ul>
}
// The `{expr}` block returns a String; it is treated as already-rendered HTML
// when the expression is a String (not escaped again).
```

---

## Component Name Mapping

PascalCase → snake_case conversion algorithm:

1. Take the first character, lowercase it.
2. For each subsequent character: if it is uppercase ASCII, prepend `_` and
   lowercase it; otherwise keep as-is.
3. Hyphens in tag names are preserved as-is (rstml parses `<My-Tag>` as a
   punctuated name; this is unusual and should be avoided).

Examples:

| PascalCase | snake_case |
|------------|-----------|
| `Button` | `button` |
| `Dialog` | `dialog` |
| `DialogContent` | `dialog_content` |
| `TabsList` | `tabs_list` |
| `ToastProvider` | `toast_provider` |
| `AlertDialogOverlay` | `alert_dialog_overlay` |

---

## Known Props vs Passthrough Attrs

The macro maintains a static table mapping component names to their known
non-`attrs` / non-`children` prop field names. For v0.1 these are the 12
components defined in `basecoat-core`. Any attribute whose key matches a known
field name is routed to the builder's typed setter; all others go into the
`AttrMap` accumulated in a `let mut __attrs = AttrMap::new()` block and passed
via `.attrs(__attrs)`.

The `children` key is never an attribute — it is always populated from the
tag body.

Fields treated as typed setters (routed by name, not through `AttrMap`):

| Component | Typed setter fields |
|-----------|-------------------|
| `Button` | `variant`, `size`, `class` |
| `Badge` | `variant`, `class` |
| `Alert` | `variant`, `class` |
| `Input` | `class` |
| `Label` | `class` |
| `Textarea` | `class` |
| `Card` | `class` |
| `Separator` | `orientation`, `class` |
| `Dialog` | `id`, `title`, `description`, `close_button`, `close_on_overlay_click`, `class` |
| `Tabs` | `default_value`, `orientation`, `class` |
| `Toast` | `id`, `title`, `description`, `category`, `duration_ms`, `closeable`, `class` |
| `Toaster` | `class` |
| `Tooltip` | `content`, `side`, `class` |

If an attribute key matches none of the above for a given component, it is
routed to `AttrMap` (passthrough). This table is hardcoded in the macro crate
and must be updated when new components or prop fields are added.
