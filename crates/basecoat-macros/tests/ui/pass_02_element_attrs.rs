// Pass: HTML element with multiple attributes.
use basecoat_macros::rsx;

fn main() {
    let markup = rsx! { <div class="foo" id="bar">"content"</div> };
    let s = markup.to_string();
    assert!(s.contains(r#"class="foo""#));
    assert!(s.contains(r#"id="bar""#));
    assert!(s.contains("content"));
}
