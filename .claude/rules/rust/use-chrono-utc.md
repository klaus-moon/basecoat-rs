---
trigger: always_on
---

Always use `chrono` with UTC for stored and computed datetimes. Never `chrono::Local`, never the `time` crate in our own code. Rendering to a user-facing timezone happens at the display layer only.

`time` may appear at FFI boundaries that mandate it (e.g. `rcgen`'s `not_before` / `not_after` fields are typed as `time::OffsetDateTime`). Convert at the boundary; keep our own logic in chrono.

Do NOT do this:

```rust
let now = chrono::Local::now();
let now = time::OffsetDateTime::now_utc();
```

Do this instead:

```rust
let now = chrono::Utc::now();
```
