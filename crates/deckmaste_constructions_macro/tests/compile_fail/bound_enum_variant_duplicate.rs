pub struct FixturePhrase;

pub enum BoundVariant {
    Phrase(FixturePhrase),
}

deckmaste_constructions_macro::constructions! {
    group duplicate_variant;
    element bound_variant bind BoundVariant {
        variant Phrase: hole FixturePhrase,
        variant Phrase: hole FixturePhrase,
    }
}

fn main() {}
