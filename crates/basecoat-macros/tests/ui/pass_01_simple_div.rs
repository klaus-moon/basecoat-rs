// Pass: simple raw HTML element renders to Markup.
use basecoat_macros::rsx;

fn main() {
    let markup = rsx! { <div>"Hello"</div> };
    let s = markup.to_string();
    assert_eq!(s, "<div>Hello</div>");
}
