pub struct FixturePhrase;

pub enum BoundVariant {
    Phrase(FixturePhrase),
}

deckmaste_constructions_macro::constructions! {
    group unknown_variant;
    element bound_variant bind BoundVariant {
        variant Missing: hole FixturePhrase,
    }
}

fn main() {}
