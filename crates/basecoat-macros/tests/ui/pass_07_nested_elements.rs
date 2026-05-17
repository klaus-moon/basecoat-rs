// Pass: nested HTML elements.
use basecoat_macros::rsx;

fn main() {
    let markup = rsx! {
        <ul class="list">
            <li>"Item 1"</li>
            <li>"Item 2"</li>
        </ul>
    };
    let s = markup.to_string();
    assert!(s.starts_with(r#"<ul class="list">"#));
    assert!(s.contains("<li>Item 1</li>"));
    assert!(s.contains("<li>Item 2</li>"));
    assert!(s.ends_with("</ul>"));
}
