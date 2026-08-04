//! Compile-fail gates: sealed generated types must reject construction and
//! mutation from outside their generated module, bad macro input must
//! produce an error spanned at the offending token, and validator
//! diagnostics must surface as `EC…` compile errors with correct spans
//! through the real `constructicon!` macro. One property per fixture file —
//! rustc suppresses the E0451 struct-literal privacy diagnostic crate-wide
//! once any other hard error exists in the same compilation unit, so a
//! fixture asserting two properties can only ever pin the survivor.
//!
//! Globbed rather than enumerated: a new fixture file under
//! `tests/compile_fail/` is picked up automatically.

#[test]
fn compile_fail_fixtures() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_fail/*.rs");
}
