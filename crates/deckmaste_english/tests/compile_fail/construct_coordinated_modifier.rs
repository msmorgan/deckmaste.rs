use deckmaste_english::syntax::CoordinatedModifier;
use deckmaste_english::syntax::ModifierCoordination;
use deckmaste_english::syntax::NominalModifier;

fn bypass_checked_coordination(
    first: Box<NominalModifier>,
    rest: Vec<ModifierCoordination>,
) -> CoordinatedModifier {
    CoordinatedModifier { first, rest }
}

fn main() {}
