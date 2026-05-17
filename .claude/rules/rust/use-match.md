---
trigger: always_on
---

Use `match` for 3+ cases over an enum or tagged union, with `{ ... }` blocks for each arm.

Do NOT do this:

```rust
if let Foo::A = x { handle_a(); }
if let Foo::B = x { handle_b(); }
if let Foo::C = x { handle_c(); }
```

Do this instead:

```rust
match x {
    Foo::A => {
        handle_a();
    }
    Foo::B => {
        handle_b();
    }
    Foo::C => {
        handle_c();
    }
}
```
