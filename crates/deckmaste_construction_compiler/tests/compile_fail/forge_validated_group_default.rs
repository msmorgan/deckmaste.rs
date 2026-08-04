use deckmaste_construction_compiler::validate::ValidatedGroup;

fn main() {
    // The emitter's input type must have no `Default` impl: an unvalidated
    // `ValidatedGroup` is meaningless, so it must be impossible to conjure
    // one out of thin air.
    let _forged: ValidatedGroup = Default::default();
}
