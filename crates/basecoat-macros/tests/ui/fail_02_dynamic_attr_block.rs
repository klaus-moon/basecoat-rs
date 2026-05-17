// Fail: dynamic attribute block syntax (`<div {"key"}>`) is not supported.
use basecoat_macros::rsx;

fn main() {
    let _ = rsx! { <div {"some-key"}>"content"</div> };
}
