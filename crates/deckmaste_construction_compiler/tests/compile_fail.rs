#[test]
fn compile_fail_fixtures() {
    let cases = trybuild::TestCases::new();
    // Two separate fixtures, not one: rustc suppresses the private-field
    // diagnostic (E0451) for a struct literal once any other hard error
    // exists anywhere in the same crate, so the `Default` failure and the
    // literal-construction failure cannot both be observed from a single
    // compilation unit. Each fixture below pins one half of the seal.
    cases.compile_fail("tests/compile_fail/forge_validated_group_default.rs");
    cases.compile_fail("tests/compile_fail/forge_validated_group_literal.rs");
}
