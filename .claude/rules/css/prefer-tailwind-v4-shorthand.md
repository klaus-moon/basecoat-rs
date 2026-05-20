Prefer Tailwind v4's canonical shorthand over the verbose legacy forms. The **Tailwind CSS IntelliSense** extension surfaces these as suggestions in the editor — apply them. This applies both to `@apply` directives in hand-written CSS (e.g. `assets/css/base.css`) and to `class="..."` strings in Rust source (`crates/**`, `examples/**`).

| Legacy form                      | Canonical v4 form     | Why                                                                   |
| -------------------------------- | --------------------- | --------------------------------------------------------------------- |
| `calc(var(--spacing)*4)`         | `--spacing(4)`        | v4 spacing function                                                   |
| `mask-[image:var(--check-icon)]` | `mask-(--check-icon)` | CSS-variable shorthand `prop-(--x)`                                   |
| `bg-[var(--brand)]`              | `bg-(--brand)`        | same shorthand, any property                                          |
| `max-h-[300px]`                  | `max-h-75`            | use the spacing scale when an exact token exists (`75 × 4px = 300px`) |
| `[&>*]:w-full`                   | `*:w-full`            | direct-children variant shorthand                                     |
| `calc(100%_-_2rem)`              | `calc(100%-2rem)`     | v4 auto-normalizes calc operators; the `_`-spaces are unnecessary     |

Do NOT do this:

```css
@apply grid-cols-[calc(var(--spacing)*4)_1fr] mask-[image:var(--check-icon)] max-h-[300px] [&>*]:w-full max-w-[calc(100%_-_2rem)];
```

Do this instead:

```css
@apply grid-cols-[--spacing(4)_1fr] mask-(--check-icon) max-h-75 *:w-full max-w-[calc(100%-2rem)];
```

The same applies to class strings in Rust:

Do NOT do this:

```rust
let body = rsx! { <main class="max-w-[calc(100%_-_2rem)] [&>*]:gap-4">"..."</main> };
```

Do this instead:

```rust
let body = rsx! { <main class="max-w-[calc(100%-2rem)] *:gap-4">"..."</main> };
```
