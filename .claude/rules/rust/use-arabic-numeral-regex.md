---
trigger: always_on
---

When matching Arabic digits, use `[0-9]` instead of `\d`. `\d` in the `regex` crate also matches other Unicode numeric forms (Eastern Arabic digits, fullwidth digits, etc.), which is almost never what we want.

Do NOT do this:

```rust
let re = regex::Regex::new(r"\d+").unwrap();
```

Do this instead:

```rust
let re = regex::Regex::new(r"[0-9]+").unwrap();
```
