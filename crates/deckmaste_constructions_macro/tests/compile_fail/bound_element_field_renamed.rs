use deckmaste_features::Comma;

pub struct FixturePhrase;

pub struct BoundMember {
    pub renamed: Comma,
    pub phrase: FixturePhrase,
}

deckmaste_constructions_macro::constructions! {
    group renamed;
    element fixture_member bind BoundMember {
        comma: lex Comma,
        phrase: hole FixturePhrase,
    }
}

fn main() {}
