//! Public sealing gates for generated English syntax.

#[test]
fn generated_syntax_cannot_bypass_checked_construction() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_fail/*.rs");
}
