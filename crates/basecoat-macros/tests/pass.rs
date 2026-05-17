// Positive trybuild tests for the rsx! macro.
//
// The pass_01 through pass_10 fixtures test only HTML element rendering and do
// not depend on `basecoat-components`.  They run unconditionally.
//
// Component-level pass tests (pass_comp_*.rs) are gated behind `full-tests`
// because they call into `::basecoat_components`, which is implemented by
// Phase 2a.  Run with: cargo test -p basecoat-macros --features full-tests
#[test]
fn pass_element_cases() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass_*.rs");
}

#[test]
#[cfg(feature = "full-tests")]
fn pass_component_cases() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass_comp_*.rs");
}
