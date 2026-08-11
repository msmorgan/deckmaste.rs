use deckmaste_english::syntax::Determiner;

fn bypass_generated_validation(determiner: &mut Determiner, replacement: Determiner) {
    determiner.0 = replacement.0;
}

fn main() {}
