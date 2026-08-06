pub struct FixturePhrase;

pub enum BoundVariant {
    Phrase(FixturePhrase),
    Other(FixturePhrase),
}

deckmaste_constructions_macro::constructions! {
    group unmapped_variant;
    element bound_variant bind BoundVariant {
        variant Phrase: hole FixturePhrase,
    }
}

fn main() {}
