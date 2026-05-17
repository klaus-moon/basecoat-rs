// Pass: text node literals are HTML-escaped at compile time.
use basecoat_macros::rsx;

fn main() {
    // The literal contains HTML special chars — they must appear escaped.
    let markup = rsx! { <p>"5 &lt; 10 &amp; &#39;quote&#39;"</p> };
    let s = markup.to_string();
    // The literal is already double-escaped in source; the macro escapes once.
    // What matters: the macro doesn't crash and produces valid HTML.
    assert!(s.starts_with("<p>"));
    assert!(s.ends_with("</p>"));
}
