// Pass: value-less (boolean) attribute on HTML element.
use basecoat_macros::rsx;

fn main() {
    let markup = rsx! { <button disabled>"Can't click"</button> };
    let s = markup.to_string();
    assert!(s.contains(" disabled"));
    // Text literal is HTML-escaped at compile time (only &, <, > in text context).
    // Apostrophe is NOT escaped in text content — only in attribute values.
    assert!(s.contains("Can't click"));
}
