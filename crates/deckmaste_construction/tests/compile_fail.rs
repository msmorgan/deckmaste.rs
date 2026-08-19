//! Real macro-boundary diagnostics for authored construction syntax.

#[test]
fn compile_fail_fixtures() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_fail/*.rs");
}
