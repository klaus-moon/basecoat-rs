// Pass: self-closing element and void element.
use basecoat_macros::rsx;

fn main() {
    // Void element — self-closing
    let hr = rsx! { <hr/> };
    assert_eq!(hr.to_string(), "<hr/>");

    // Non-void self-closing is equivalent to empty children
    let span = rsx! { <span/> };
    assert_eq!(span.to_string(), "<span></span>");
}
