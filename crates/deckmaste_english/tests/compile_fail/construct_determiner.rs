use deckmaste_english::syntax::Determiner;
use deckmaste_english::syntax::DeterminerKind;

fn bypass_generated_validation() -> Determiner {
    Determiner(DeterminerKind::The)
}

fn main() {}
