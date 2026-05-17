// Pass: fragment produces concatenated children with no wrapper.
use basecoat_macros::rsx;

fn main() {
    let markup = rsx! {
        <>
            <li>"First"</li>
            <li>"Second"</li>
        </>
    };
    let s = markup.to_string();
    assert_eq!(s, "<li>First</li><li>Second</li>");
}
