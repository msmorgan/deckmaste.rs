use deckmaste_english::syntax::Determiner;

fn bypass_generated_validation(determiner: &mut Determiner, replacement: Determiner) {
    determiner.repr = replacement.repr;
}

fn main() {}
