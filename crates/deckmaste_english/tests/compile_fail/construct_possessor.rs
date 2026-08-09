use deckmaste_english::syntax::Possessor;

fn bypass_generated_validation(source: Possessor) -> Possessor {
    Possessor { repr: source.repr }
}

fn main() {}
