// Pass: expression block as child — value is emitted raw (no auto-escaping).
// Caller is responsible for escaping user-controlled data.
// Nested rsx!{} calls return Markup whose Display is already-safe HTML.
use basecoat_macros::rsx;

fn main() {
    // Plain string: emitted raw — angle brackets pass through unchanged.
    let name = "World";
    let markup = rsx! { <p>{name}</p> };
    let s = markup.to_string();
    assert_eq!(s, "<p>World</p>");

    // Nested rsx! — inner Markup Display emits HTML correctly without
    // double-escaping.
    let inner = rsx! { <b>"bold"</b> };
    let outer = rsx! { <p>{inner}</p> };
    assert_eq!(outer.to_string(), "<p><b>bold</b></p>");
}
