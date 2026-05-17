---
trigger: always_on
---

Prefer early return and the `?` operator over nested `if`/`if let`/`match` pyramids.

Do NOT do this:

```rust
fn process(opt_x: Option<i32>, f: impl Fn(i32) -> Option<i32>) -> Result<i32, Error> {
    if let Some(x) = opt_x {
        if let Some(y) = f(x) {
            return Ok(y + 1);
        }
    }
    Err(Error::Missing)
}
```

Do this instead:

```rust
fn process(opt_x: Option<i32>, f: impl Fn(i32) -> Option<i32>) -> Result<i32, Error> {
    let x = opt_x.ok_or(Error::Missing)?;
    let y = f(x).ok_or(Error::Missing)?;
    Ok(y + 1)
}
```
