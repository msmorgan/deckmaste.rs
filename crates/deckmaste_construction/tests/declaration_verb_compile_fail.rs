//! Focused declaration-backed verb recipe diagnostics.

#[test]
fn declaration_verb_compile_fail_fixtures() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_fail/declaration_verb_*.rs");
}
