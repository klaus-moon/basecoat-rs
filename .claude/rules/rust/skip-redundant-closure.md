---
trigger: always_on
---

If a `fn` matches the callback signature, pass the fn directly — don't wrap it in a closure. The Clippy lint `clippy::redundant_closure` is set to `deny` in this workspace.

Do NOT do this:

```rust
vec![1, 2, 3].into_iter().map(|x| Foo::new(x)).collect::<Vec<_>>();
```

Do this instead:

```rust
vec![1, 2, 3].into_iter().map(Foo::new).collect::<Vec<_>>();
```
