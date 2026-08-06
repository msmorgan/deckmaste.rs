pub struct FixturePhrase;
pub struct OtherPhrase;

pub enum BoundVariant {
    Phrase(OtherPhrase),
}

deckmaste_constructions_macro::constructions! {
    group payload_drift;
    element bound_variant bind BoundVariant {
        variant Phrase: hole FixturePhrase,
    }
}

fn main() {}
