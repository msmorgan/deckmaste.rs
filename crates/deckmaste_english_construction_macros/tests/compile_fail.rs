//! Compile-fail gates: sealed generated types must reject construction and
//! mutation from outside their generated module, and bad macro input must
//! produce an error spanned at the offending token.

#[test]
fn compile_fail_fixtures() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_fail/construct_private_field.rs");
    cases.compile_fail("tests/compile_fail/mutate_private_field.rs");
    cases.compile_fail("tests/compile_fail/bad_macro_input.rs");
}
