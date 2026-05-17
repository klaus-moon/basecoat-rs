// Fail: void element with children must produce a compile error.
use basecoat_macros::rsx;

fn main() {
    let _ = rsx! { <br>"oops"</br> };
}
