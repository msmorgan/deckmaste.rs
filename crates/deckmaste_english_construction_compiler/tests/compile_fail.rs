#[test]
fn compile_fail_fixtures() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_fail/forge_validated_group.rs");
}
