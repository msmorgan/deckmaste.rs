use deckmaste_english_construction_compiler::validate::ValidatedGroup;

fn main() {
    // The emitter's input type must be unconstructible outside the
    // validator. Both literal construction and Default must fail.
    let _forged: ValidatedGroup = Default::default();
}
