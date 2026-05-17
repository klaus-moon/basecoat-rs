// Fail: closure binding attribute syntax (`<div on(x)>`) is not supported.
use basecoat_macros::rsx;

fn main() {
    let _ = rsx! { <div on(x)>"content"</div> };
}
