// Negative trybuild tests for the rsx! macro.
//
// These test that the macro produces clear compile errors for invalid inputs.
// They do NOT require basecoat-components, so they run without the
// `full-tests` feature.
#[test]
fn fail_cases() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail_*.rs");
}
