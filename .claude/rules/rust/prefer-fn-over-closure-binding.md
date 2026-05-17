---
trigger: always_on
---

Don't bind a closure to a `let` at module scope; declare it as a `fn`. Inline closures inside method chains (`.map`, `.filter`, `.and_then`) are fine — this rule targets module-scope bindings only.

Do NOT do this:

```rust
let add_one = |x: i32| x + 1;
static DOUBLE: fn(i32) -> i32 = |x| x * 2;
```

Do this instead:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn double(x: i32) -> i32 {
    x * 2
}
```
