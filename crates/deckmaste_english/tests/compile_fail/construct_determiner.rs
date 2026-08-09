use deckmaste_english::syntax::Determiner;

fn bypass_generated_validation(source: Determiner) -> Determiner {
    Determiner { repr: source.repr }
}

fn main() {}
