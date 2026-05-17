// Pass: HTML element with an expression attribute value.
use basecoat_macros::rsx;

fn main() {
    let my_class = "dynamic";
    let markup = rsx! { <span class={my_class}>"text"</span> };
    let s = markup.to_string();
    assert!(s.contains(r#"class="dynamic""#));
}
