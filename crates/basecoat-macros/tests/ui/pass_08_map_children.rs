// Pass: using a Rust iterator expression as children (list rendering pattern).
use basecoat_macros::rsx;

fn main() {
    let items = vec!["alpha", "beta", "gamma"];
    let markup = rsx! {
        <ul>
            { items.iter().map(|i| rsx!{ <li>{i}</li> }.to_string()).collect::<String>() }
        </ul>
    };
    let s = markup.to_string();
    assert!(s.contains("<li>alpha</li>"));
    assert!(s.contains("<li>beta</li>"));
    assert!(s.contains("<li>gamma</li>"));
}
