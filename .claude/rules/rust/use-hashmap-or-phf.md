---
trigger: always_on
---

For key-to-value dispatch, use a lookup map — `phf` for `'static` data, `HashMap` for dynamic. Do not use if-chains or a `match` that returns only literals.

Do NOT do this:

```rust
let weight = match kind {
    "a" => 1,
    "b" => 2,
    "c" => 3,
    _ => 0,
};
```

Do this instead (static):

```rust
static WEIGHTS: phf::Map<&'static str, u32> = phf::phf_map! {
    "a" => 1,
    "b" => 2,
    "c" => 3,
};

let weight = WEIGHTS.get(kind).copied().unwrap_or(0);
```

Do this instead (dynamic):

```rust
let weights: HashMap<&str, u32> = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);
let weight = weights.get(kind).copied().unwrap_or(0);
```
