---
trigger: always_on
---

Replace try/catch-IIFE patterns with `Result`/`Option` combinators and the `?` operator.

Do NOT do this:

```rust
let value = (|| -> Result<i32, Error> {
    let x = compute()?;
    Ok(x + 1)
})()
.unwrap_or(0);
```

Do this instead:

```rust
let value = compute().map(|x| x + 1).unwrap_or(0);
```

Or, when you want to propagate:

```rust
let value = compute()?.checked_add(1).ok_or(Error::Overflow)?;
```
