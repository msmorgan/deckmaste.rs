//! Public sealing gates for generated coordination syntax.

#[test]
fn coordination_syntax_cannot_bypass_generated_validation() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_fail/*.rs");
}
