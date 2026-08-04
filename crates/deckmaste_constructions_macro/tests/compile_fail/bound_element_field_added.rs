use deckmaste_features::Comma;

pub struct FixturePhrase;

pub struct BoundMember {
    pub comma: Comma,
    pub phrase: FixturePhrase,
    pub added: FixturePhrase,
}

deckmaste_constructions_macro::constructions! {
    group added;
    element fixture_member bind BoundMember {
        comma: lex Comma,
        phrase: hole FixturePhrase,
    }
}

fn main() {}
