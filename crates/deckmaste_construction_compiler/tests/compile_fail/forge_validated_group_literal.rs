use deckmaste_construction_compiler::model::GroupDeclaration;
use deckmaste_construction_compiler::model::Spanned;
use deckmaste_construction_compiler::validate::ValidatedGroup;

fn main() {
    // The emitter's input type must be unconstructible outside the
    // validator: `ValidatedGroup`'s `group` field is private to
    // `validate.rs`, so a literal construction naming it must fail even
    // when the `GroupDeclaration` behind it is entirely well-formed.
    let group = GroupDeclaration {
        name: Spanned::call_site(String::new()),
        backend: deckmaste_construction_compiler::model::ConstructionBackend::Chart,
        constructions: vec![],
        elements: vec![],
        lenses: vec![],
    };
    let _literal = ValidatedGroup { group: &group };
}
